pub mod helpers;
pub mod plugins;
pub mod prelude;

use std::collections::HashMap;

use minifb::{Window, WindowOptions};
use serde_json::Value;

pub use minifb;

/// The base game structure.
pub struct Game {
	pub state: HashMap<String, Value>,
	pub window: Window,
	pub res: (usize, usize),
	pub fb: Vec<u32>,
	pub tick: Box<dyn Fn(&mut HashMap<String, Value>)>,
	pub draw: Box<dyn Fn(&mut Window, &mut Vec<u32>, &mut HashMap<String, Value>)>,
	pub plugins: Vec<Plugin>,
}

/// Plugins execute extra tick and draw functions.
/// A plugin could, for example, be used to draw sprites,
/// keep track of time, or perform actions in the background.
pub struct Plugin {
	pub priority: isize,
	pub tick: Box<dyn Fn(&mut HashMap<String, Value>)>,
	pub draw: Box<dyn Fn(&mut Window, &mut Vec<u32>, &mut HashMap<String, Value>)>,
}

impl<'a> Game {
	/// Build a new game structure. This creates an empty window.
	pub fn new(title: String, res: (usize, usize), rate: u64) -> Game {
		let mut window = Window::new(&title, res.0, res.1, WindowOptions::default())
			.expect("Couldn't init window");
		window.limit_update_rate(Some(std::time::Duration::from_micros(1000000 / rate)));
		let buf = vec![0; res.0 * res.1];
		Game {
			state: HashMap::default(),
			window: window,
			res,
			fb: buf,
			tick: Box::new(|_| {}),
			draw: Box::new(|_, _, _| {}),
			plugins: vec![],
		}
	}
	/// Write the tick function for use in builder patterns.
	pub fn with_tick(mut self, tick: Box<dyn Fn(&mut HashMap<String, Value>)>) -> Self {
		self.tick = tick;
		self
	}
	/// Write the draw function for use in builder patterns.
	pub fn with_draw(
		mut self,
		draw: Box<dyn Fn(&mut Window, &mut Vec<u32>, &mut HashMap<String, Value>)>,
	) -> Self {
		self.draw = draw;
		self
	}
	/// Add a plugin for use in builder patterns.
	pub fn with_plugin(mut self, plugin: Plugin) -> Self {
		self.plugins.push(plugin);
		self
	}
	/// Sort the list of plugins so they will execute in the right order.
	pub fn sort_plugins(&mut self) {
		self.plugins.sort_by(|a, b| a.priority.cmp(&b.priority));
	}
	/// Run the tick functions.
	pub fn tick(&mut self) {
		for plugin in &self.plugins {
			(plugin.tick)(&mut self.state);
		}
		(self.tick)(&mut self.state);
	}
	/// Run the draw functions and refresh the screen.
	pub fn draw(&mut self) {
		for plugin in &self.plugins {
			(plugin.draw)(&mut self.window, &mut self.fb, &mut self.state);
		}
		(self.draw)(&mut self.window, &mut self.fb, &mut self.state);
		self.window
			.update_with_buffer(&self.fb, self.res.0, self.res.1)
			.unwrap();
	}
}
