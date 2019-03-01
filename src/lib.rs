#[derive(Debug)]
struct Player {
	health: u32,
	mana: u32,
	x: u32,
	y: u32, 
}

#[derive(Debug)]
enum TileType {
	Floor,
	Wall,
}

#[derive(Debug)]
struct Tile {
	tile_type: TileType,
	row: u16,
	column: u16
}

impl Player {
	pub fn new(health: u32, mana: u32, x: u32, y: u32) -> Self {
		Player {
			health,
			mana,
			x,
			y
		}
	}

	fn load_asset() {

	}
}

trait Drawable {
	fn draw(&self);
}

impl Drawable for Player {
	fn draw(&self) {

	}
}