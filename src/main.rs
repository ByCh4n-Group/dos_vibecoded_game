mod bios;
mod boot;
mod menu;
mod game;

use tetra::{Context, ContextBuilder, State};
use bios::BiosState;
use boot::BootState;
use menu::MenuState;
use game::GameState;

enum GamePhase {
    Bios(BiosState),
    Boot(BootState),
    Menu(MenuState),
    Game(GameState),
}

struct MainState {
    phase: GamePhase,
}

impl MainState {
    fn new(ctx: &mut Context) -> tetra::Result<MainState> {
        // Start with BIOS state
        let bios_state = BiosState::new(ctx)?;
        Ok(MainState {
            phase: GamePhase::Bios(bios_state),
        })
    }
}

impl State for MainState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        match &mut self.phase {
            GamePhase::Bios(state) => {
                state.update(ctx)?;
                if state.is_finished() {
                    let boot_state = BootState::new(ctx)?;
                    self.phase = GamePhase::Boot(boot_state);
                }
            }
            GamePhase::Boot(state) => {
                state.update(ctx)?;
                if state.is_finished() {
                    let menu_state = MenuState::new(ctx)?;
                    self.phase = GamePhase::Menu(menu_state);
                }
            }
            GamePhase::Menu(state) => {
                state.update(ctx)?;
                if state.should_start_game {
                    let game_state = GameState::new(ctx)?;
                    self.phase = GamePhase::Game(game_state);
                }
            }
            GamePhase::Game(state) => {
                state.update(ctx)?;
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        match &mut self.phase {
            GamePhase::Bios(state) => state.draw(ctx),
            GamePhase::Boot(state) => state.draw(ctx),
            GamePhase::Menu(state) => state.draw(ctx),
            GamePhase::Game(state) => state.draw(ctx),
        }
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("DOS Vibecoded Game", 640, 480)
        .show_mouse(true)
        .build()?
        .run(MainState::new)
}
