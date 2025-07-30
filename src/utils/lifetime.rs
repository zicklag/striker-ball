use bones_framework::prelude::*;
use std::time::Duration;

#[derive(Default)]
pub struct LifetimePlugin;
impl SessionPlugin for LifetimePlugin {
    fn install(self, session: &mut SessionBuilder) {
        // [`Entities::clear_killed`] in [`World::maintain`] in
        // [`SystemStages::run`] is called after running systems
        // so we check lifetime expirations in `Last` so they are
        // removed immediately afterward.
        session.add_system_to_stage(Last, Self::update_lifetimes);
    }
}
impl LifetimePlugin {
    pub fn update_lifetimes(
        time: Res<Time>,
        mut entities: ResMut<Entities>,
        mut lifetimes: CompMut<Lifetime>,
    ) {
        // TODO: Update Bones to be able to handle this without an allocation.
        let dead: Vec<Entity> = entities
            .iter_with(&mut lifetimes)
            .filter_map(|(entity, lifetime)| {
                lifetime.tick(time.delta());
                lifetime.dead().then_some(entity)
            })
            .collect();

        for entity in dead {
            entities.kill(entity);
        }
    }
}

#[derive(HasSchema, Clone, Copy)]
pub struct Lifetime {
    pub target: Duration,
    pub elapsed: Duration,
}
impl Default for Lifetime {
    fn default() -> Self {
        Self::seconds(1.0)
    }
}
impl Lifetime {
    pub fn new(duration: Duration) -> Self {
        Lifetime {
            target: duration,
            elapsed: default(),
        }
    }
    pub fn seconds(seconds: f32) -> Self {
        Lifetime {
            target: Duration::from_secs_f32(seconds),
            elapsed: default(),
        }
    }
    pub fn tick(&mut self, elapsed: Duration) {
        self.elapsed += elapsed;
    }
    pub fn dead(&self) -> bool {
        self.elapsed >= self.target
    }
}
