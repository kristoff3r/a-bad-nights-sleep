pub use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Component)]
pub struct Timed(pub f32);

pub(crate) struct TimedEntityPlugin;

impl Plugin for TimedEntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_timed_entities);
    }
}

fn update_timed_entities(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Timed)>,
    time: Res<Time>,
) {
    for (entity, mut timed) in query.iter_mut() {
        timed.0 -= time.delta_secs();
        if timed.0 < 0.0 {
            debug!(target: "timed_entity", "Despawning timed {entity:?}");
            commands.entity(entity).despawn_recursive();
        }
    }
}
