use super::*;

#[derive(HasSchema, Clone, Default)]
#[repr(C)]
pub struct WinnerBannerAssets {
    pub team_a: SizedImageAsset,
    pub team_b: SizedImageAsset,
}

#[derive(HasSchema, Clone, Default)]
pub struct WinnerBanner {
    pub visual: Visual,
    pub team: Team,
    pub timer: Timer,
}
impl SessionPlugin for WinnerBanner {
    fn install(self, session: &mut SessionBuilder) {
        session.insert_resource(self);
        session.add_system_to_stage(First, |world: &World, time: Res<Time>| {
            world.resource_mut::<Self>().timer.tick(time.delta());
        });
    }
}
pub fn show(world: &World) {
    let winner = world.resource::<WinnerBanner>();

    if !winner.visual.shown() {
        return;
    }
    let asset_server = world.resource::<AssetServer>();
    let root = asset_server.root::<Data>();

    use egui::*;
    Area::new("match-done-ui")
        .anchor(Align2::CENTER_CENTER, [0., 0.])
        .show(&world.resource::<EguiCtx>(), |ui| {
            let egui_textures = world.resource::<EguiTextures>();
            ui.horizontal(|ui| {
                let banner = match winner.team {
                    Team::A => &root.menu.winner_banner.team_a,
                    Team::B => &root.menu.winner_banner.team_b,
                };
                ui.image(ImageSource::Texture(load::SizedTexture::new(
                    egui_textures.get(**banner),
                    banner.egui_size(),
                )));
            });
        });
}
