pub mod character;
pub mod day;
pub mod effects;
pub mod enemy;
pub mod night;
pub mod player;
pub mod timed_entity;

use avian2d::prelude::*;
use bevy::{
    asset::AssetMetaCheck,
    core_pipeline::{bloom::Bloom, tonemapping::Tonemapping},
    prelude::*,
};
use bevy_hanabi::HanabiPlugin;
use character::CharacterControllerPlugin;
use day::DayPlugin;
use effects::EffectsPlugin;
use enemy::EnemyPlugin;
use night::NightPlugin;
use player::PlayerPlugin;
use timed_entity::TimedEntityPlugin;
use vleue_navigator::VleueNavigatorPlugin;

#[derive(PhysicsLayer, Default)]
enum GameLayer {
    #[default]
    Default,
    Player,
    Enemy,
}

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(AssetPlugin {
            meta_check: AssetMetaCheck::Never,
            ..default()
        }),
        PhysicsPlugins::default().with_length_unit(20.0),
        VleueNavigatorPlugin,
        HanabiPlugin,
        CharacterControllerPlugin,
        TimedEntityPlugin,
        EffectsPlugin,
        PlayerPlugin,
        EnemyPlugin,
        NightPlugin,
        DayPlugin,
    ))
    .init_state::<GameState>()
    .enable_state_scoped_entities::<GameState>()
    .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.1)))
    .add_systems(Startup, game_setup);

    #[cfg(feature = "dev")]
    app.add_plugins(avian2d::debug_render::PhysicsDebugPlugin::default())
        .add_systems(Update, print_collisions);

    app.run();
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]
    DayTime,
    NightTime,
    GameOver,
    GameWon,
}

fn game_setup(mut commands: Commands) {
    // Camera
    commands.spawn((
        Camera2d,
        Camera {
            hdr: true,
            ..default()
        },
        Tonemapping::TonyMcMapface,
        Bloom::default(),
    ));
}

fn print_collisions(mut collision_event_reader: EventReader<Collision>) {
    for Collision(contacts) in collision_event_reader.read() {
        println!(
            "Entities {} and {} are colliding",
            contacts.entity1, contacts.entity2,
        );
    }
}
