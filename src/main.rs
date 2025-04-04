pub mod character;
mod level;
mod player;

use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_hanabi::HanabiPlugin;
use character::CharacterControllerPlugin;
use level::LevelPlugin;
use player::PlayerPlugin;
use vleue_navigator::VleueNavigatorPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default().with_length_unit(20.0),
            VleueNavigatorPlugin,
            HanabiPlugin,
            CharacterControllerPlugin,
            PlayerPlugin,
            LevelPlugin,
        ))
        .init_state::<GameState>()
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.1)))
        .add_systems(Startup, game_setup)
        .run();
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    DayTime,
    #[default]
    NightTime,
}

fn game_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Camera
    commands.spawn(Camera2d);
}
