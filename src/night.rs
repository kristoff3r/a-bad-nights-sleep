use bevy::{
    math::{vec2, vec3},
    prelude::*,
    time::Stopwatch,
};
use vleue_navigator::NavMesh;

use crate::{
    enemy::{EnemyRules, EnemySpawner},
    player::PlayerStats,
    GameState,
};

pub struct NightPlugin;

#[derive(Resource)]
pub struct LevelState {
    pub timer: Stopwatch,
}

impl Plugin for NightPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Level>();
        app.add_systems(OnEnter(GameState::NightTime), load_level);

        app.add_systems(Update, level_time.run_if(in_state(GameState::NightTime)));
    }
}

#[derive(Resource, Default)]
pub struct Level {
    pub navmesh: Handle<NavMesh>,
    pub spawners: Vec<EnemySpawner>,
    pub rules: Handle<EnemyRules>,
}

fn load_level(
    mut commands: Commands,
    mut navmeshes: ResMut<Assets<NavMesh>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut level: ResMut<Level>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    level.navmesh = navmeshes.add(NavMesh::from_edge_and_obstacles(vec![], vec![]));
    level.rules = asset_server.load("enemies.ron");

    commands.spawn((
        StateScoped(GameState::NightTime),
        EnemySpawner {
            start_pos: vec2(200.0, 0.0),
            spawn_rate: 1.0,
            spawn_count: 5,
            radius: 10.0,
        },
        Transform::from_translation(vec3(200.0, 0.0, 0.0)),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
    ));

    commands.insert_resource(LevelState {
        timer: Stopwatch::new(),
    });
}

fn level_time(
    mut level_state: ResMut<LevelState>,
    time: Res<Time>,
    player_stats: Res<PlayerStats>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    level_state.timer.tick(time.delta());
    if level_state.timer.elapsed_secs() > player_stats.sleep_duration {
        next_state.set(GameState::DayTime);
    }
}
