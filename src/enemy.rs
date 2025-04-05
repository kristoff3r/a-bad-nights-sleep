use avian2d::prelude::{Collider, Collision, CollisionLayers, LinearVelocity, RigidBody};
use bevy::{math::vec2, prelude::*};
use bevy_common_assets::ron::RonAssetPlugin;
use fastrand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    effects::{self, Effects},
    night::Level,
    player::NightPlayer,
    timed_entity::Timed,
    GameLayer, GameState,
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<EnemyRules>::new(&["enemies.ron"]));
        app.add_event::<EnemyDiedEvent>();
        app.add_systems(
            Update,
            (
                spawn_enemies,
                (player_collisions, handle_enemy_death).chain(),
            )
                .run_if(in_state(GameState::NightTime)),
        );
    }
}

#[derive(Event)]
pub struct EnemyDiedEvent {
    pub entity: Entity,
    pub transform: GlobalTransform,
}

#[derive(Component)]
#[require(Enemy)]
pub enum EnemyType {
    Basic,
}

#[derive(Component, Default)]
#[require(LastSpawnTime)]
pub struct EnemySpawner {
    pub start_pos: Vec2,
    pub spawn_rate: f32,
    pub spawn_count: u32,
    pub radius: f32,
}

#[derive(Component, Default, Deref, DerefMut)]
pub struct LastSpawnTime(f32);

#[derive(Component, Default, Clone, Copy)]
pub struct Enemy;

#[derive(Asset, TypePath, Serialize, Deserialize)]
pub struct EnemyRules {
    pub base_speed: f32,
    pub health: f32,
}

fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut spawner_query: Query<(&EnemySpawner, &mut LastSpawnTime, &Transform)>,
    rules: Res<Assets<EnemyRules>>,
    level: Res<Level>,
    player_query: Query<&Transform, With<NightPlayer>>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    let Some(rules) = rules.get(&level.rules) else {
        return;
    };

    let cur_time = time.elapsed_secs();
    let mut rng = Rng::new();

    let material = materials.add(Color::srgb(7.0, 0.2, 0.2));

    let radius = 7.0;
    let mesh = meshes.add(Circle::new(radius));

    for (enemy_spawner, mut last_spawn_time, transform) in spawner_query.iter_mut() {
        if last_spawn_time.0 + enemy_spawner.spawn_rate.recip() <= cur_time {
            for _ in 0..enemy_spawner.spawn_count {
                let rx = rng.f32();
                let ry = rng.f32();
                let pos = transform.translation.truncate()
                    + vec2(
                        rx * enemy_spawner.radius * 2.0 - enemy_spawner.radius,
                        ry * enemy_spawner.radius * 2.0 - enemy_spawner.radius,
                    );

                let direction = (player_transform.translation.truncate() - pos).normalize();
                commands.spawn((
                    Enemy,
                    StateScoped(GameState::NightTime),
                    Transform::from_translation(pos.extend(0.0)),
                    Mesh2d(mesh.clone()),
                    Collider::circle(radius),
                    CollisionLayers::new(GameLayer::Enemy, [GameLayer::Default, GameLayer::Player]),
                    RigidBody::Dynamic,
                    MeshMaterial2d(material.clone()),
                    LinearVelocity(rules.base_speed * direction),
                ));
            }
            **last_spawn_time = cur_time;
        }
    }
}

fn player_collisions(
    mut collision_event_reader: EventReader<Collision>,
    enemies: Query<(Entity, &GlobalTransform), With<Enemy>>,
    mut enemy_died_writer: EventWriter<EnemyDiedEvent>,
) {
    for Collision(contacts) in collision_event_reader.read() {
        if contacts.collision_started() {
            let Ok((enemy_entity, enemy_transform)) = enemies
                .get(contacts.entity1)
                .or_else(|_| enemies.get(contacts.entity2))
            else {
                continue;
            };

            enemy_died_writer.send(EnemyDiedEvent {
                entity: enemy_entity,
                transform: enemy_transform.clone(),
            });
        }
    }
}

fn handle_enemy_death(
    mut commands: Commands,
    mut player_query: Query<&mut NightPlayer>,
    mut enemy_died_event_reader: EventReader<EnemyDiedEvent>,
    effects: Res<Effects>,
) {
    let Ok(mut player) = player_query.get_single_mut() else {
        return;
    };

    for &EnemyDiedEvent { entity, transform } in enemy_died_event_reader.read() {
        commands.entity(entity).despawn_recursive();
        commands.spawn((
            StateScoped(GameState::NightTime),
            Timed(0.5),
            Transform::from_translation(transform.translation()),
            effects.death_effect.clone(),
        ));
        player.health -= 1.0;
    }
}
