use tetra::graphics::{self, Color};
use tetra::graphics::text::{Font, Text};
use tetra::math::Vec2;
use tetra::{Context, State};

pub struct GameState {
    text: Text,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let font = Font::vector(ctx, "./resources/DOS_font.ttf", 40.0)?;
        let text = Text::new("GAME STARTED", font);

        Ok(GameState {
            text,
        })
    }

    pub fn update(&mut self, _ctx: &mut Context) -> tetra::Result {
        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.1, 0.1, 0.1));
        self.text.draw(ctx, Vec2::new(200.0, 220.0));
        Ok(())
    }
}
