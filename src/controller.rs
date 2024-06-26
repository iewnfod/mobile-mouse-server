use enigo::{Enigo, Mouse, Settings};

pub struct Controller {
	enigo: Enigo
}

impl Controller {
	pub fn new() -> Self {
		Self {
			enigo: Enigo::new(&Settings::default()).unwrap()
		}
	}

	pub fn move_mouse(&mut self, delta_x: i64, delta_y: i64) {
		self.enigo.move_mouse(delta_x as i32, delta_y as i32, enigo::Coordinate::Rel).unwrap();
	}
}
