use bevy::prelude::*;
use bevy_hanabi::prelude::*;

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Effects>();
    }
}

#[derive(Resource)]
pub struct Effects {
    pub death_effect: ParticleEffect,
}

impl FromWorld for Effects {
    fn from_world(world: &mut World) -> Self {
        let mut effects = world.resource_mut::<Assets<EffectAsset>>();
        let death_effect = ParticleEffect::new(effects.add(death_effect()));

        Self { death_effect }
    }
}

fn death_effect() -> EffectAsset {
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

    let spawner = SpawnerSettings::rate(300.0.into());
    EffectAsset::new(1024, spawner, module)
        .with_name("death effect")
        .init(init_pos)
        .init(init_vel)
        .init(init_age)
        .init(init_lifetime)
        .render(SizeOverLifetimeModifier {
            gradient: Gradient::constant(Vec3::splat(10.0)),
            screen_space_size: false,
        })
        .render(ColorOverLifetimeModifier::new(gradient))
        .render(round)
}
