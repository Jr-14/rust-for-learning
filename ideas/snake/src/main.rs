use ggez::glam::{vec2, Vec2};
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};

struct GameState {
    pos_x: f32,
    circle: graphics::Mesh,
}

impl GameState {
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            vec2(0., 0.),
            100.0,
            2.0,
            Color::WHITE,
        )?;
        Ok(GameState { pos_x: 0.0, circle })
    }
}

impl EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        canvas.draw(&self.circle, Vec2::new(self.pos_x, 380.0));

        canvas.finish(ctx)?;

        Ok(())
    }
}

fn main() -> GameResult {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "JR")
        .build()
        .expect("aiieee, could not create ggez context!");

    let state = GameState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}

