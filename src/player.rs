use avian2d::prelude::{Collider, CollisionLayers, Sensor};
use bevy::{color::palettes::tailwind, prelude::*};

use crate::{character::CharacterControllerBundle, night::LevelState, GameLayer, GameState};
use bevy_hanabi::prelude::*;

pub struct PlayerPlugin;

#[derive(Resource)]
pub struct PlayerStats {
    pub comfort: f32,
    pub snug: f32,
    pub warmth: f32,
    pub hydration: f32,
    pub sleep_duration: f32,
    pub sleep_intensity: f32,
    pub rest: f32,
    pub unsafe_rest: f32,
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
            rest: 200.0,
            unsafe_rest: 0.0,
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
    mut effects: ResMut<Assets<EffectAsset>>,
    player_stats: Res<PlayerStats>,
) {
    // Create a color gradient for the particles
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(0.5, 0.5, 1.0, 1.0));
    gradient.add_key(1.0, Vec4::new(0.5, 0.5, 1.0, 0.0));

    let writer = ExprWriter::new();

    let age = writer.lit(0.).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    let lifetime = writer.lit(2.).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let init_pos = SetPositionCircleModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::Z).expr(),
        radius: writer.lit(10.0).expr(),
        dimension: ShapeDimension::Surface,
    };

    let init_vel = SetVelocityCircleModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::Z).expr(),
        speed: writer.lit(40.0).expr(),
    };

    let mut module = writer.finish();

    let round = RoundModifier::constant(&mut module, 2.0 / 3.0);

    // Create a new effect asset spawning 30 particles per second from a circle
    // and slowly fading from blue-ish to transparent over their lifetime.
    // By default the asset spawns the particles at Z=0.
    let spawner = SpawnerSettings::rate(50.0.into());
    let effect = effects.add(
        EffectAsset::new(4096, spawner, module)
            .with_name("2d")
            .init(init_pos)
            .init(init_vel)
            .init(init_age)
            .init(init_lifetime)
            .render(SizeOverLifetimeModifier {
                gradient: Gradient::constant(Vec3::splat(10.0)),
                screen_space_size: false,
            })
            .render(ColorOverLifetimeModifier::new(gradient))
            .render(round),
    );

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
    // player.with_child((ParticleEffect::new(effect), Name::new("effect:2d")));
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
    time: Res<Time>,
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
