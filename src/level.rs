use avian2d::prelude::{Collider, LinearVelocity, RigidBody};
use bevy::{
    math::{vec2, vec3},
    prelude::*,
};
use fastrand::Rng;
use serde::{Deserialize, Serialize};
use vleue_navigator::NavMesh;

use crate::{player::NightPlayer, GameState};

pub struct LevelPlugin;

#[derive(Component)]
#[require(Enemy)]
pub enum EnemyType {
    Basic,
}

#[derive(Component, Default, Serialize, Deserialize)]
#[require(LastSpawnTime)]
pub struct EnemySpawner {
    pub spawn_rate: f32,
    pub spawn_count: u32,
    pub radius: f32,
}

#[derive(Component, Default, Deref, DerefMut)]
pub struct LastSpawnTime(f32);

#[derive(Component, Default, Clone, Copy)]
pub struct Enemy;

#[derive(Resource, Default, Serialize, Deserialize)]
pub struct LevelState {
    #[serde(skip)]
    pub navmesh: Handle<NavMesh>,
    pub spawners: Vec<EnemySpawner>,
}

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LevelState>()
            .add_systems(OnEnter(GameState::NightTime), load_level)
            .add_systems(Update, (spawn_enemies).chain().after(spawn_enemy_spawner));
    }
}

fn load_level(
    mut commands: Commands,
    mut navmeshes: ResMut<Assets<NavMesh>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut level: ResMut<LevelState>,
) {
    level.navmesh = navmeshes.add(NavMesh::from_edge_and_obstacles(vec![], vec![]));

    commands.spawn((
        EnemySpawner {
            spawn_rate: 1.0,
            spawn_count: 5,
            radius: 10.0,
        },
        Transform::from_translation(vec3(0.0, 0.0, 0.0)),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
    ));
}

fn spawn_enemy_spawner(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
}

fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut spawner_query: Query<(&EnemySpawner, &mut LastSpawnTime, &Transform)>,
    player_query: Query<&Transform, With<NightPlayer>>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    let cur_time = time.elapsed_secs();
    let mut rng = Rng::new();

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
                let radius = 7.0;
                commands.spawn((
                    Enemy,
                    Transform::from_translation(pos.extend(0.0)),
                    Mesh2d(meshes.add(Circle::new(radius))),
                    Collider::circle(radius),
                    RigidBody::Dynamic,
                    MeshMaterial2d(materials.add(Color::srgb(1.0, 0.2, 0.2))),
                    LinearVelocity(150.0 * direction),
                ));
            }
            **last_spawn_time = cur_time;
        }
    }
}
