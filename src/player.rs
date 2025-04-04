use avian2d::prelude::Collider;
use bevy::prelude::*;

use crate::character::CharacterControllerBundle;
use bevy_hanabi::prelude::*;

pub struct PlayerPlugin;

#[derive(Resource, Default)]
pub struct PlayerState {
    comfort: f32,
    warmth: f32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerState>()
            .add_systems(Startup, spawn_player);
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    // Spawn a reference white square in the center of the screen at Z=0
    commands.spawn((
        Mesh2d(meshes.add(Rectangle {
            half_size: Vec2::splat(0.1),
        })),
        MeshMaterial2d(materials.add(ColorMaterial {
            color: Color::WHITE,
            ..Default::default()
        })),
        Name::new("square"),
    ));

    // Create a color gradient for the particles
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(0.5, 0.5, 1.0, 1.0));
    gradient.add_key(1.0, Vec4::new(0.5, 0.5, 1.0, 0.0));

    let writer = ExprWriter::new();

    let age = writer.lit(0.).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    let lifetime = writer.lit(5.).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let init_pos = SetPositionCircleModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::Z).expr(),
        radius: writer.lit(0.05).expr(),
        dimension: ShapeDimension::Surface,
    };

    let init_vel = SetVelocityCircleModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::Z).expr(),
        speed: writer.lit(0.1).expr(),
    };

    let mut module = writer.finish();

    let round = RoundModifier::constant(&mut module, 2.0 / 3.0);

    // Create a new effect asset spawning 30 particles per second from a circle
    // and slowly fading from blue-ish to transparent over their lifetime.
    // By default the asset spawns the particles at Z=0.
    let spawner = SpawnerSettings::rate(30.0.into());
    let effect = effects.add(
        EffectAsset::new(4096, spawner, module)
            .with_name("2d")
            .init(init_pos)
            .init(init_vel)
            .init(init_age)
            .init(init_lifetime)
            .render(SizeOverLifetimeModifier {
                gradient: Gradient::constant(Vec3::splat(0.02)),
                screen_space_size: false,
            })
            .render(ColorOverLifetimeModifier::new(gradient))
            .render(round),
    );

    // Spawn an instance of the particle effect, and override its Z layer to
    // be above the reference white square previously spawned.
    commands.spawn((
        ParticleEffect::new(effect),
        Name::new("effect:2d"),
        CharacterControllerBundle::new(Collider::capsule(12.5, 20.0)).with_movement(1250.0, 0.92),
    ));

    // commands.spawn((
    //     Mesh2d(meshes.add(Capsule2d::new(12.5, 20.0))),
    //     MeshMaterial2d(materials.add(Color::srgb(0.2, 0.7, 0.9))),
    //     Transform::from_xyz(0.0, -100.0, 0.0),
    //     CharacterControllerBundle::new(Collider::capsule(12.5, 20.0)).with_movement(1250.0, 0.92),
    // ));
}
