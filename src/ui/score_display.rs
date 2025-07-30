use super::*;

#[derive(HasSchema, Clone, Default)]
pub struct ScoreDisplay {
    pub visual: Visual,
    pub timer: Timer,
}
impl ScoreDisplay {
    pub fn new(secs: f32) -> ScoreDisplay {
        let mut timer = Timer::from_seconds(secs, TimerMode::Once);
        timer.pause();
        ScoreDisplay {
            visual: Visual::default(),
            timer,
        }
    }
    pub fn restart(&mut self) {
        self.timer.reset();
        self.timer.unpause();
        self.visual.show();
    }
}
impl SessionPlugin for ScoreDisplay {
    fn install(self, session: &mut SessionBuilder) {
        session.insert_resource(self);
        session.add_system_to_stage(First, |world: &World, time: Res<Time>| {
            world.resource_mut::<Self>().timer.tick(time.delta());
        });
    }
}
pub fn show(world: &World) {
    let mut score_display = world.resource_mut::<ScoreDisplay>();
    if !score_display.visual.shown() {
        return;
    }
    let score = world.resource::<PinScore>();
    let asset_server = world.resource::<AssetServer>();
    let root = asset_server.root::<Data>();
    let ctx = world.resource::<EguiCtx>();
    let textures = world.resource::<EguiTextures>();

    let Menus {
        numbers, score_bg, ..
    } = root.menu;

    if !score_display.timer.finished() {
        use egui::*;
        Area::new("scoreboard")
            .order(Order::Foreground)
            .anchor(Align2::CENTER_CENTER, [0., 0.])
            .show(&ctx, |ui| {
                let width = score_bg.egui_size().x;
                let height = score_bg.egui_size().y;

                ui.set_width(width);
                ui.set_height(height);

                ui.painter().image(
                    textures.get(*score_bg),
                    Rect::from_min_size(ui.cursor().min, Vec2::new(width, height)),
                    default_uv(),
                    Color32::WHITE,
                );

                let asset = asset_server.get(numbers);
                let width = asset.tile_size.x;
                let height = asset.tile_size.y;

                let shift = 1.0 / asset.rows as f32;

                ui.painter().image(
                    textures.get(asset.image),
                    Rect::from_min_size(ui.cursor().min, Vec2::new(width, height)),
                    Rect::from_min_max(
                        pos2(0.0, shift * score.a as f32),
                        pos2(1.0, shift * score.a as f32 + shift),
                    ),
                    Color32::WHITE,
                );

                ui.painter().image(
                    textures.get(asset.image),
                    Rect::from_min_size(
                        ui.cursor().min + ui.available_size() - Vec2::new(width, height),
                        Vec2::new(width, height),
                    ),
                    Rect::from_min_max(
                        pos2(0.0, shift * score.b as f32),
                        pos2(1.0, shift * score.b as f32 + shift),
                    ),
                    Color32::WHITE,
                );
            });
    } else {
        score_display.visual.hide();
    }
}
