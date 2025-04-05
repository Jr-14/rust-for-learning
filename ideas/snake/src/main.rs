use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};

struct GameState {

}


fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "JR")
        .build()
        .expect("aiieee, could not create ggez context!");

    let my_game = GameState::new(&mut ctx);
}

impl GameState {
    pub fn new(_ctx: &mut Context) -> GameResult {
        GameState {

        }
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        canvas.finish(ctx)
    }
}