use crate::{
	helpers::{get_state, put_state},
	Plugin,
};

/// Adds a state key named "timer" that increments every frame.
pub fn frame_counter() -> Plugin {
	Plugin {
		priority: 0,
		tick: Box::new(|state| {
			let timer: Option<usize> = get_state(state, "frame_counter");
			let timer = match timer {
				Some(v) => v + 1,
				None => 0,
			};
			put_state(state, "frame_counter", timer)
		}),
		draw: Box::new(|_, _, _| {}),
	}
}
