use super::*;

#[derive(HasSchema, Clone, Default)]
#[repr(C)]
pub struct MatchDoneAssets {
    pub buttons: Handle<Image>,
    pub button_picker1: SizedImageAsset,
    pub button_picker2: SizedImageAsset,
}

#[derive(HasSchema, Clone, Default, Copy, Deref, DerefMut)]
pub struct MatchDone {
    pub visual: Visual,
    #[deref]
    pub state: MatchDoneState,
}
#[derive(HasSchema, Clone, Default, Copy)]
pub enum MatchDoneState {
    #[default]
    PlayAgain,
    TeamSelect,
}
impl MatchDone {
    pub fn toggle(&mut self) {
        self.state = match self.state {
            MatchDoneState::PlayAgain => MatchDoneState::TeamSelect,
            MatchDoneState::TeamSelect => MatchDoneState::PlayAgain,
        }
    }
}
impl SessionPlugin for MatchDone {
    fn install(self, session: &mut SessionBuilder) {
        session.insert_resource(self);
    }
}
pub fn show(world: &World) {
    let match_done = world.resource::<MatchDone>();

    if !match_done.visual.shown() {
        return;
    }
    let asset_server = world.resource::<AssetServer>();
    let root = asset_server.root::<Data>();
    let MatchDoneAssets {
        button_picker1,
        button_picker2,
        buttons,
        ..
    } = root.menu.match_done;

    use egui::*;
    Area::new("match-done-ui")
        .anchor(Align2::CENTER_CENTER, [0., 0.])
        .show(&world.resource::<EguiCtx>(), |ui| {
            let textures = world.resource::<EguiTextures>();
            ui.horizontal(|ui| {
                ui.style_mut().spacing.item_spacing = Vec2::ZERO;
                ui.image(ImageSource::Texture(load::SizedTexture::new(
                    match match_done.state {
                        MatchDoneState::PlayAgain => textures.get(*button_picker1),
                        MatchDoneState::TeamSelect => textures.get(*button_picker2),
                    },
                    button_picker1.egui_size(),
                )));
                ui.image(ImageSource::Texture(load::SizedTexture::new(
                    textures.get(buttons),
                    [55., 30.],
                )));
            });
        });
}
