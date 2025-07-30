use super::*;

#[derive(HasSchema, Clone, Copy, Default)]
pub enum MenuState {
    #[default]
    Splash,
    HowToPlay,
    FadeToTeamSelect,
    TeamSelect,
    FadeToGame,
}
pub struct MenuPlugin;
impl SessionPlugin for MenuPlugin {
    fn install(self, session: &mut SessionBuilder) {
        session.init_resource::<MenuState>();

        session.install_plugin(Splash::Offline);
        session.install_plugin(HowToPlay::default());
        session.install_plugin(Fade::new(0.7, 0.5, Color::BLACK, egui::Order::Foreground));
        session.install_plugin(TeamSelect::default());
        session.install_plugin(Pause::default());
        session.add_startup_system(|root: Root<Data>, mut audio: ResMut<AudioCenter>| {
            audio.play_music_advanced(
                *root.sound.menu_music,
                root.sound.menu_music.volume(),
                true,
                false,
                0.0,
                1.0,
                true,
            );
        });
        session.add_system_to_stage(First, update_menu);
        session.add_system_to_stage(First, update_pause);
    }
}

pub fn update_pause(ui: &World) {
    if ui
        .resource_mut::<Sessions>()
        .get_mut(session::PLAY)
        .is_none()
    {
        return;
    };
    let mut pause = ui.resource_mut::<Pause>();
    let local_inputs = ui.resource::<LocalInputs>();

    let unpause = || {
        let mut sessions = ui.resource_mut::<Sessions>();
        let session = sessions.get_mut(session::PLAY).unwrap();
        session
            .world
            .resource_mut::<Countdown>()
            .visual
            .remove_hide();
        session
            .world
            .resource_mut::<MatchDone>()
            .visual
            .remove_hide();
        session
            .world
            .resource_mut::<ScoreDisplay>()
            .visual
            .remove_hide();
        session
            .world
            .resource_mut::<WinnerBanner>()
            .visual
            .remove_hide();
        session.active = true;
    };

    for (_gamepad, input) in local_inputs.iter() {
        if input.down.just_pressed() {
            pause.cycle()
        }
        if input.up.just_pressed() {
            pause.cycle();
            pause.cycle();
        }
        if input.start.just_pressed() {
            match *pause {
                Pause::Hidden => {
                    let mut sessions = ui.resource_mut::<Sessions>();
                    let session = sessions.get_mut(session::PLAY).unwrap();
                    session.world.resource_mut::<Countdown>().visual.add_hide();
                    session.world.resource_mut::<MatchDone>().visual.add_hide();
                    session
                        .world
                        .resource_mut::<ScoreDisplay>()
                        .visual
                        .add_hide();
                    session
                        .world
                        .resource_mut::<WinnerBanner>()
                        .visual
                        .add_hide();
                    session.active = false;
                    *pause = Pause::Continue;
                }
                Pause::Continue | Pause::Restart | Pause::Quit => {
                    unpause();
                    *pause = Pause::Hidden;
                }
            }
        }
        if input.south.just_pressed() {
            match *pause {
                Pause::Continue => {
                    unpause();
                    *pause = Pause::Hidden;
                }
                Pause::Restart => {
                    ui.resource_mut::<Fade>().restart();
                    *ui.resource_mut() = MenuState::FadeToGame;
                    *pause = Pause::Hidden;
                }
                Pause::Quit => {
                    *ui.resource_mut::<Splash>() = Splash::Offline;
                    *ui.resource_mut() = MenuState::Splash;
                    ui.resource_mut::<Sessions>().delete_play();
                    *pause = Pause::Hidden;
                }
                Pause::Hidden => {}
            }
        }
    }
}

pub fn update_menu(world: &World) {
    let game_state = *world.resource::<MenuState>();
    match game_state {
        MenuState::Splash => splash_update(world),
        MenuState::HowToPlay => how_to_play_update(world),
        MenuState::FadeToTeamSelect => fade_to_team_select_update(world),
        MenuState::TeamSelect => team_select_update(world),
        MenuState::FadeToGame => fade_to_game_update(world),
    }
}

pub fn splash_update(ui: &World) {
    let mut state = ui.resource_mut::<MenuState>();
    let mut fade = ui.resource_mut::<Fade>();
    let mut splash = ui.resource_mut::<Splash>();
    let mut howtoplay = ui.resource_mut::<HowToPlay>();

    let inputs = ui.resource::<LocalInputs>();

    for (_gamepad, input) in inputs.iter() {
        if input.up.just_pressed() {
            splash.cycle_up();
        }
        if input.down.just_pressed() {
            splash.cycle_down();
        }
        if input.south.just_pressed() {
            match *splash {
                Splash::Offline => {
                    fade.restart();
                    *state = MenuState::FadeToTeamSelect;
                    return;
                }
                // Splash::Online => todo!(),
                Splash::HowToPlay => {
                    *splash = Splash::Hidden;
                    *state = MenuState::HowToPlay;
                    *howtoplay = HowToPlay::GameOverview;
                    return;
                }
                Splash::Hidden => todo!(),
            }
        }
    }
}
pub fn how_to_play_update(ui: &World) {
    let mut state = ui.resource_mut::<MenuState>();
    let mut splash = ui.resource_mut::<Splash>();
    let mut howtoplay = ui.resource_mut::<HowToPlay>();

    let inputs = ui.resource::<LocalInputs>();

    for (_gamepad, input) in inputs.iter() {
        if input.west.just_pressed() {
            *howtoplay = HowToPlay::Hidden;
            *splash = Splash::Offline;
            *state = MenuState::Splash;
        }
        match *howtoplay {
            HowToPlay::GameOverview => {
                if input.right.just_pressed() {
                    *howtoplay = HowToPlay::SingleStickControls;
                }
            }
            HowToPlay::DualStickControls => {
                if input.left.just_pressed() {
                    *howtoplay = HowToPlay::SingleStickControls;
                }
            }
            HowToPlay::SingleStickControls => {
                if input.left.just_pressed() {
                    *howtoplay = HowToPlay::GameOverview;
                }
                if input.right.just_pressed() {
                    *howtoplay = HowToPlay::DualStickControls;
                }
            }
            HowToPlay::Hidden => {}
        }
    }
}
pub fn fade_to_team_select_update(ui: &World) {
    let fade = ui.resource::<Fade>();
    let mut state = ui.resource_mut::<MenuState>();
    let mut splash = ui.resource_mut::<Splash>();
    let mut team_select = ui.resource_mut::<TeamSelect>();

    if fade.fade_out.just_finished() {
        tracing::info!("fade out, show team select");
        *splash = Splash::Hidden;
        team_select.visible = true;
    }
    if fade.fade_in.just_finished() {
        tracing::info!("fade in, control team select");
        *state = MenuState::TeamSelect;
    }
}
pub fn team_select_update(ui: &World) {
    let assignments = ui.resource_mut::<TeamSelect>().get_player_signs();
    let local_inputs = ui.resource::<LocalInputs>();
    let asset_server = ui.asset_server();
    let root = asset_server.root::<Data>();

    for (gamepad, input) in local_inputs.iter() {
        if input.start.just_pressed() && assignments.is_some() {
            ui.resource_mut::<Fade>().restart();
            *ui.resource_mut() = MenuState::FadeToGame;
            return;
        }
        if input.start.just_pressed()
            || input.north.just_pressed()
            || input.east.just_pressed()
            || input.south.just_pressed()
            || input.west.just_pressed()
            || input.left_bump.just_pressed()
            || input.right_bump.just_pressed()
        {
            ui.resource_mut::<TeamSelect>().add_gamepad(*gamepad);
            ui.resource_mut::<GamepadsRumble>().set_rumble(
                *gamepad,
                GamepadRumbleIntensity::LIGHT_BOTH,
                0.2,
            );
        }
        if input.south.just_pressed() {
            ui.resource_mut::<TeamSelect>().ready_gamepad(*gamepad);
        }
        if input.west.just_pressed() {
            ui.resource_mut::<TeamSelect>().reverse_gamepad(*gamepad);
        }
        if input.west.just_held(root.menu.team_select.back_buffer) {
            *ui.resource_mut::<TeamSelect>() = default();
            *ui.resource_mut::<Splash>() = Splash::Offline;
            *ui.resource_mut() = MenuState::Splash
        }
        if input.left.just_pressed() {
            ui.resource_mut::<TeamSelect>().left_gamepad(*gamepad);
        }
        if input.right.just_pressed() {
            ui.resource_mut::<TeamSelect>().right_gamepad(*gamepad);
        }
        if input.right_bump.just_held(20) && input.left_bump.just_held(20) {
            ui.resource_mut::<Fade>().restart();
            *ui.resource_mut() = MenuState::FadeToGame;
            return;
        }
    }
}

fn fade_to_game_update(ui: &World) {
    let Fade {
        fade_out, fade_in, ..
    } = &*ui.resource::<Fade>();

    let mut sessions = ui.resource_mut::<Sessions>();

    if fade_out.just_finished() {
        let player_signs = ui
            .resource::<TeamSelect>()
            .get_player_signs()
            .unwrap_or_else(|| {
                tracing::warn!(
                    "gamepad assignments were not made, defaulting to id 0 for all players"
                );
                default()
            });

        tracing::info!("fade_out, recreating PLAY session; assignments:{player_signs:?}");

        *ui.resource_mut::<TeamSelect>() = default();
        sessions.delete_play();
        sessions.create_play(PlayMode::Offline(player_signs));
    }

    if fade_in.just_finished() {
        tracing::info!("fade_in, starting countdown");
        sessions
            .get_world(session::PLAY)
            .unwrap()
            .resource_mut::<Countdown>()
            .restart();
    }
}
