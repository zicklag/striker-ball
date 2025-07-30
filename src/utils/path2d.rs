use bones_framework::prelude::*;

/// Returns the points necessary for a [`Path2d::points`] to display
/// a rectangle around the center.
pub fn rect_points(Vec2 { x, y }: Vec2) -> Vec<Vec2> {
    vec![
        Vec2::new(-x, -y),
        Vec2::new(-x, y),
        Vec2::new(x, y),
        Vec2::new(x, -y),
        Vec2::new(-x, -y),
    ]
}
/// Returns the points necessary for a [`Path2d::points`] to display
/// a circle around the center with the chosen amount of lines making it up.
pub fn circle_points(radius: f32, lines: usize) -> Vec<Vec2> {
    let mut vec = Vec::new();
    let start = Vec2::X;
    let rotate = 360. / lines as f32;
    for n in 0..=lines {
        vec.push(start.rotate(Vec2::from_angle((rotate * n as f32).to_radians())) * radius);
    }
    vec
}

#[derive(HasSchema, Clone)]
#[schema(no_default)]
pub struct Path2dToggle {
    pub hide: bool,
    swapped: bool,
    swap: AtomicComponentStore<Path2d>,
}
impl Path2dToggle {
    pub fn shown() -> Self {
        Path2dToggle {
            hide: false,
            swapped: false,
            swap: AtomicComponentStore::new(AtomicCell::new(ComponentStore::default())),
        }
    }
    pub fn hidden() -> Self {
        Path2dToggle {
            hide: true,
            swapped: false,
            swap: AtomicComponentStore::new(AtomicCell::new(ComponentStore::default())),
        }
    }
    fn unswap_path2ds(world: &World, mut storage: ResMut<Path2dToggle>) {
        if storage.hide && storage.swapped {
            storage.swap.swap(&world.components.get_cell::<Path2d>());
            storage.swapped = false;
        }
    }
    fn apply_visibility(world: &World, mut storage: ResMut<Path2dToggle>) {
        if storage.hide && !storage.swapped {
            storage.swap.swap(&world.components.get_cell::<Path2d>());
            storage.swapped = true;
        }
    }
}
impl SessionPlugin for Path2dToggle {
    fn install(self, session: &mut SessionBuilder) {
        session.insert_resource(self);
        session
            // TODO: Maybe add custom SystemStage to ensure early and late activation.
            .add_system_to_stage(First, Self::unswap_path2ds)
            .add_system_to_stage(Last, Self::apply_visibility);
    }
}
