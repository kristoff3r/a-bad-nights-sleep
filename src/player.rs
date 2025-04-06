use avian2d::prelude::{Collider, CollisionLayers, LinearVelocity, RigidBody, Sensor};
use bevy::{color::palettes::tailwind, prelude::*};

use crate::{
    character::CharacterControllerBundle, enemy::Enemy, night::LevelState, GameLayer, GameState,
};

pub struct PlayerPlugin;

#[derive(Resource, PartialEq, Clone)]
pub struct PlayerStats {
    pub comfort: f32,
    pub snug: f32,
    pub warmth: f32,
    pub hydration: f32,
    pub sleep_duration: f32,
    pub rest: u32,
    pub unsafe_rest: u32,
    pub day: u32,
    pub died: bool,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            comfort: 5.0,
            snug: 0.0,
            warmth: 0.0,
            hydration: 0.0,
            sleep_duration: 15.0,
            rest: 300,
            unsafe_rest: 0,
            day: 0,
            died: false,
        }
    }
}

#[derive(Component)]
pub struct NightPlayer {
    pub speed: f32,
    pub health: f32,
    pub last_shot: f32,
}

#[derive(Component)]
pub struct PlayerShot;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerStats>();

        app.add_systems(
            OnEnter(GameState::NightTime),
            (spawn_night_player, spawn_hud),
        );

        app.add_systems(
            Update,
            (update_hud, player_shoot, player_death).run_if(in_state(GameState::NightTime)),
        );
    }
}

fn spawn_night_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player_stats: Res<PlayerStats>,
) {
    let radius = 12.5 + player_stats.comfort / 5.0;
    let speed = (300.0 + player_stats.hydration * 20.0).max(0.0);
    commands.spawn((
        Sensor,
        NightPlayer {
            speed: player_stats.hydration,
            health: player_stats.comfort,
            last_shot: 0.0,
        },
        CollisionLayers::new(GameLayer::Player, [GameLayer::Default, GameLayer::Enemy]),
        CharacterControllerBundle::new(Collider::circle(radius)).with_movement(speed, 0.92),
        StateScoped(GameState::NightTime),
        Mesh2d(meshes.add(Circle::new(radius))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.7, 3.5))),
    ));
}

fn player_shoot(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    time: Res<Time>,
    enemies: Query<&GlobalTransform, With<Enemy>>,
    player_stats: Res<PlayerStats>,
    mut player_query: Query<(&mut NightPlayer, &Transform)>,
) {
    let Ok((mut player, player_transform)) = player_query.get_single_mut() else {
        return;
    };

    if player.last_shot + (player_stats.warmth + 1.0).recip() > time.elapsed_secs() {
        return;
    }

    let mut closest_enemy: Option<&GlobalTransform> = None;
    let mut closest_enemy_distance = f32::MAX;
    for enemy in enemies.iter() {
        let distance = enemy.translation().distance(player_transform.translation);
        if closest_enemy.is_none() || distance < closest_enemy_distance {
            closest_enemy = Some(enemy);
            closest_enemy_distance = distance;
        }
    }

    if let Some(enemy_transform) = closest_enemy {
        let distance = enemy_transform
            .translation()
            .distance(player_transform.translation);
        if distance < 100.0 + (player_stats.warmth / 10.0) {
            let radius = 3.0;
            let material = materials.add(Color::srgb(0.0, 0.2, 10.2));
            let mesh = meshes.add(Circle::new(radius));
            let direction =
                (enemy_transform.translation() - player_transform.translation).normalize();

            commands.spawn((
                PlayerShot,
                Transform::from_translation(player_transform.translation),
                Mesh2d(mesh.clone()),
                Collider::circle(radius),
                Sensor,
                CollisionLayers::new(GameLayer::Player, [GameLayer::Default, GameLayer::Enemy]),
                RigidBody::Dynamic,
                MeshMaterial2d(material.clone()),
                LinearVelocity(direction.truncate() * 1000.0),
            ));
        }

        player.last_shot = time.elapsed_secs();
    }
}

fn player_death(
    player_query: Query<(&NightPlayer, &GlobalTransform)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut player_stats: ResMut<PlayerStats>,
) {
    let Ok((player, transform)) = player_query.get_single() else {
        return;
    };

    if transform.translation().distance(Vec3::ZERO) > 2000.0 {
        info!("player fell off the map");
        player_stats.died = true;
        next_state.set(GameState::DayTime);
    }

    if player.health <= 0.0 {
        info!("player died");
        player_stats.died = true;
        next_state.set(GameState::DayTime);
    }
}

#[derive(Component)]
struct HudSleepTimer;

#[derive(Component)]
struct HudHealth;

fn spawn_hud(mut commands: Commands) {
    commands.spawn((
        HudSleepTimer,
        StateScoped(GameState::NightTime),
        Node {
            justify_self: JustifySelf::Start,
            align_self: AlignSelf::FlexEnd,
            height: Val::Px(10.0),
            width: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(tailwind::RED_700.into()),
    ));

    commands.spawn((
        HudHealth,
        StateScoped(GameState::NightTime),
        Text::new("Comfort: 4"),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(15.0),
            ..default()
        },
    ));
}

fn update_hud(
    mut sleep_timer_query: Query<&mut Node, With<HudSleepTimer>>,
    mut health_query: Query<&mut Text, With<HudHealth>>,
    level_state: Res<LevelState>,
    player_stats: Res<PlayerStats>,
    player: Query<&NightPlayer>,
) {
    let Ok(mut node) = sleep_timer_query.get_single_mut() else {
        return;
    };

    let percent = 100.0 * level_state.timer.elapsed_secs() / player_stats.sleep_duration;
    node.width = Val::Percent(100.0 - percent);

    let Ok(player) = player.get_single() else {
        return;
    };

    let Ok(mut text) = health_query.get_single_mut() else {
        return;
    };
    text.0 = format!(
        "Comfort: {:.0} Rest: {}",
        player.health,
        player_stats.unsafe_rest + level_state.timer.elapsed_secs() as u32
    );
}
