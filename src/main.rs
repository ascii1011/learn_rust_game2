use ggez;
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};
//use ggez::input::keyboard::{self, KeyCode};
//use rand::{self, thread_rng, Rng};

struct MainState {
    score: i32,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> Self {
        //let (screen_w, screen_h) = graphics::drawable_size(ctx);
        //let (screen_w_half, screen_h_half) = (screen_w * 0.5, screen_h * 0.5);

        MainState {
            score: 0,
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        //let dt = ggez::timer::delta(ctx).as_secs_f32();
        //let (screen_w, screen_h) = graphics::drawable_size(ctx);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::BLACK);
        //let screen_w = graphics::drawable_size(ctx).0;
        //let screen_w_half = screen_w * 0.5;

        let draw_param = graphics::DrawParam::default();
        let score_text = graphics::Text::new(format!("blah blah"));
        graphics::draw(ctx, &score_text, draw_param)?;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {

    let (mut ctx, event_loop) = ggez::ContextBuilder::new("silly_wars", "Chris Harty")
        .build()
        .expect("aieee, could not create ggez context!");
    
    let my_game = MainState::new(&mut ctx);
    event::run(ctx, event_loop, my_game);

}
