use enigo::{Enigo, Mouse, Settings};

pub enum MouseKey {
	Left,
	Right
}

pub struct Controller {
	enigo: Enigo
}

impl Controller {
	pub fn new() -> Self {
		Self {
			enigo: Enigo::new(&Settings::default()).unwrap()
		}
	}

	pub fn move_mouse(&mut self, delta_x: f64, delta_y: f64) {
		self.enigo.move_mouse(delta_x as i32, delta_y as i32, enigo::Coordinate::Rel).unwrap();
	}

	pub fn click(&mut self, mouse_key: MouseKey) {
		match mouse_key {
			MouseKey::Left => self.enigo.button(enigo::Button::Left, enigo::Direction::Click),
			MouseKey::Right => self.enigo.button(enigo::Button::Right, enigo::Direction::Click)
		}.unwrap();
	}

	pub fn press(&mut self, mouse_key: MouseKey) {
		match mouse_key {
			MouseKey::Left => self.enigo.button(enigo::Button::Left, enigo::Direction::Press),
			MouseKey::Right => self.enigo.button(enigo::Button::Right, enigo::Direction::Press)
		}.unwrap();
	}

	pub fn release(&mut self, mouse_key: MouseKey) {
		match mouse_key {
			MouseKey::Left => self.enigo.button(enigo::Button::Left, enigo::Direction::Release),
			MouseKey::Right => self.enigo.button(enigo::Button::Right, enigo::Direction::Release)
		}.unwrap();
	}
}
