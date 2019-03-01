use std::rc:Rc;
use sprite::*;

#[derive(Debug)]
struct SpriteSheet {
	src: Rc<Texture>,
	sprite_dimensions: (u32, u32),
}

impl SpriteSheet {
	pub fn new(src: Rc<Texture>, sprite_dimensions: (u32, u32)) {
		SpriteSheet {
			src,
			sprite_dimensions
		}
	}
}