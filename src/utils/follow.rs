use bones_framework::prelude::*;

#[derive(HasSchema, Clone)]
pub enum Follow {
    XYZ { target: Entity, offset: Vec3 },
    XY { target: Entity, offset: Vec2 },
    X { target: Entity, offset: f32 },
    Y { target: Entity, offset: f32 },
}
impl Default for Follow {
    fn default() -> Self {
        Self::XYZ {
            target: default(),
            offset: default(),
        }
    }
}
impl Follow {
    pub fn target(&self) -> Entity {
        match self {
            Self::XYZ { target, .. }
            | Self::XY { target, .. }
            | Self::X { target, .. }
            | Self::Y { target, .. } => *target,
        }
    }
}

pub struct FollowPlugin;
impl SessionPlugin for FollowPlugin {
    fn install(self, session: &mut SessionBuilder) {
        session.add_system_to_stage(First, Self::update);
    }
}
impl FollowPlugin {
    pub fn update(
        entities: Res<Entities>,
        follow: Comp<Follow>,
        mut transforms: CompMut<Transform>,
    ) {
        for (bound, follow) in entities.iter_with(&follow) {
            if let Some(target) = transforms.get_mut(follow.target()).cloned() {
                if let Some(bound) = transforms.get_mut(bound) {
                    match follow {
                        Follow::XYZ { offset, .. } => {
                            bound.translation = target.translation + *offset
                        }
                        Follow::XY { offset, .. } => {
                            bound.translation.x = target.translation.x + offset.x;
                            bound.translation.y = target.translation.y + offset.y;
                        }
                        Follow::X { offset, .. } => {
                            bound.translation.x = target.translation.x + offset
                        }
                        Follow::Y { offset, .. } => {
                            bound.translation.y = target.translation.y + offset
                        }
                    }
                }
            }
        }
    }
}
