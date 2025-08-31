#![allow(clippy::too_many_arguments)]
use bones_bevy_renderer::BonesBevyRenderer;
use bones_framework::prelude::*;
use striker_ball::*;

fn main() {
    setup_logs!("studio", "ktech", "striker_ball");

    crate::register_schemas();

    let mut game = Game::new();

    game.install_plugin(DefaultGamePlugin);
    game.init_shared_resource::<AssetServer>();
    // By inserting `ClearColor` as a shared resource, every session
    // will by default read its own `ClearColor` as `BLACK`.
    game.insert_shared_resource(ClearColor(Color::BLACK));

    game.install_plugin(LocalInputGamePlugin);
    game.sessions.create_with(session::UI, UiSessionPlugin);

    BonesBevyRenderer::new(game)
        .namespace(("studio", "ktech", "striker_ball"))
        .preload(true)
        .app()
        .run();
}
