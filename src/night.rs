use bevy::{math::vec2, prelude::*, time::Stopwatch};
use fastrand::Rng;
use vleue_navigator::NavMesh;

use crate::{
    enemy::{Enemy, EnemyRules, EnemySpawner, EnemyType},
    player::PlayerStats,
    GameState,
};

pub struct NightPlugin;

#[derive(Resource)]
pub struct LevelState {
    pub timer: Stopwatch,
    pub last_spawn: f32,
}

impl Plugin for NightPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Level>();
        app.add_systems(OnEnter(GameState::NightTime), load_level);

        app.add_systems(
            Update,
            (level_time, spawn_enemies).run_if(in_state(GameState::NightTime)),
        );
    }
}

#[derive(Resource, Default)]
pub struct Level {
    pub navmesh: Handle<NavMesh>,
    pub rules: Handle<EnemyRules>,
}

fn load_level(
    mut commands: Commands,
    mut navmeshes: ResMut<Assets<NavMesh>>,
    mut level: ResMut<Level>,
    mut player_stats: ResMut<PlayerStats>,
    asset_server: Res<AssetServer>,
) {
    player_stats.unsafe_rest = 0;
    player_stats.died = false;
    level.navmesh = navmeshes.add(NavMesh::from_edge_and_obstacles(vec![], vec![]));
    level.rules = asset_server.load("enemies.ron");

    commands.insert_resource(LevelState {
        timer: Stopwatch::new(),
        last_spawn: 0.0,
    });
}

fn level_time(
    mut level_state: ResMut<LevelState>,
    time: Res<Time>,
    mut player_stats: ResMut<PlayerStats>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    level_state.timer.tick(time.delta());
    if level_state.timer.elapsed_secs() > player_stats.sleep_duration {
        info!("Sleep duration elapsed");
        player_stats.unsafe_rest += level_state.timer.elapsed_secs() as u32;
        next_state.set(GameState::DayTime);
    }
}

fn spawn_enemies(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut level_state: ResMut<LevelState>,
) {
    let mut rng = Rng::new();
    let cur_time = level_state.timer.elapsed_secs();
    for (t, spawn) in SPAWNS.iter() {
        if cur_time >= *t && level_state.last_spawn < *t {
            let mesh = meshes.add(Rectangle::new(20.0, 20.0));
            let material = materials.add(Color::srgb(1.0, 1.0, 1.0));
            for _ in 0..spawn.count {
                let spawner = EnemySpawner {
                    spawn_rate: spawn.spawn_rate,
                    radius: 10.0,
                    spawn_type: EnemyType::default(),
                };

                let rx = rng.f32() - 0.5;
                let ry = rng.f32() - 0.5;
                let pos = vec2(rx * 1500.0, ry * 800.0);
                let transform = Transform::from_translation(pos.extend(0.0));
                commands.spawn((
                    StateScoped(GameState::NightTime),
                    Enemy,
                    spawner,
                    transform,
                    MeshMaterial2d(material.clone()),
                    Mesh2d(mesh.clone()),
                ));
            }
            level_state.last_spawn = *t;
        }
    }
}

pub struct Spawn {
    spawn_rate: f32,
    count: u32,
}

pub const SPAWNS: &[(f32, Spawn)] = &[
    (
        1.0,
        Spawn {
            spawn_rate: 0.3,
            count: 2,
        },
    ),
    (
        5.0,
        Spawn {
            spawn_rate: 0.5,
            count: 3,
        },
    ),
    (
        20.0,
        Spawn {
            spawn_rate: 0.6,
            count: 3,
        },
    ),
    (
        30.0,
        Spawn {
            spawn_rate: 1.5,
            count: 6,
        },
    ),
    (
        40.0,
        Spawn {
            spawn_rate: 5.0,
            count: 2,
        },
    ),
    (
        45.0,
        Spawn {
            spawn_rate: 5.0,
            count: 7,
        },
    ),
    (
        50.0,
        Spawn {
            spawn_rate: 5.0,
            count: 15,
        },
    ),
];
