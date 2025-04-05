use avian2d::prelude::{Collider, CollisionLayers, Sensor};
use bevy::{color::palettes::tailwind, prelude::*};

use crate::{
    character::CharacterControllerBundle, effects::Effects, night::LevelState, GameLayer, GameState,
};
use bevy_hanabi::prelude::*;

pub struct PlayerPlugin;

#[derive(Resource, PartialEq, Clone)]
pub struct PlayerStats {
    pub comfort: f32,
    pub snug: f32,
    pub warmth: f32,
    pub hydration: f32,
    pub sleep_duration: f32,
    pub sleep_intensity: f32,
    pub rest: u32,
    pub unsafe_rest: f32,
    pub day: u32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            comfort: 5.0,
            snug: 0.0,
            warmth: 0.0,
            hydration: 0.0,
            sleep_duration: 5.0,
            sleep_intensity: 1.0,
            rest: 200,
            unsafe_rest: 0.0,
            day: 0,
        }
    }
}

#[derive(Component)]
pub struct NightPlayer {
    pub speed: f32,
    pub health: f32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerStats>();

        app.add_systems(
            OnEnter(GameState::NightTime),
            (spawn_night_player, spawn_hud),
        );

        app.add_systems(Update, update_hud.run_if(in_state(GameState::NightTime)));
    }
}

fn spawn_night_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player_stats: Res<PlayerStats>,
) {
    let radius = 12.5;
    let mut player = commands.spawn((
        Sensor,
        NightPlayer {
            speed: player_stats.hydration,
            health: player_stats.comfort,
        },
        CollisionLayers::new(GameLayer::Player, [GameLayer::Default, GameLayer::Enemy]),
        CharacterControllerBundle::new(Collider::circle(radius)).with_movement(1250.0, 0.92),
        StateScoped(GameState::NightTime),
        Mesh2d(meshes.add(Circle::new(radius))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.7, 3.5))),
    ));
}

#[derive(Component)]
struct HudSleepTimer;

#[derive(Component)]
struct HudHealth;

fn spawn_hud(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
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
    text.0 = format!("Comfort: {:.2}", player.health);
}
