use mini_game::prelude::*;
use minifb::Key;

fn main() {
	let mut game = Game::new("Hello, world!".to_string(), (480, 360), 60)
		.with_plugin(plugins::frame_counter())
		.with_tick(Box::new(|_| {}))
		.with_draw(Box::new(|_, fb, state| {
			let timer = helpers::get_state(state, "frame_counter").unwrap();
			for (index, pixel) in fb.iter_mut().enumerate() {
				if index < timer {
					*pixel = u32::MAX;
				}
			}
		}));
	loop {
		game.tick();
		game.draw();
		if game.window.is_key_down(Key::Escape) {
			break;
		}
	}
}
