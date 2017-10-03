
#[derive(Copy, Clone)]
pub enum Tile {
	Water,
	Land
}


pub struct World {
	width: u32,
	height: u32,
	tiles: Vec<Tile>
}

impl World {
	pub fn new(width: u32, height: u32) -> World {
		let mut world = World {
			width,
			height,
			tiles: Vec::new()
		};
		world.init();
		world
	}

	pub fn set_tile(&mut self, x: u32, y: u32, tile: Tile) {
		let index = self.tile_index(x, y);
		self.tiles[index] = tile;
	}

	pub fn get_tile(&self, x: u32, y: u32) -> Tile {
		self.tiles[self.tile_index(x, y)]
	}

	fn tile_index(&self, x: u32, y: u32) -> usize{
		(y * self.width + x) as usize
	}

	fn init(&mut self) {
		for index in 0..(self.width*self.height){
			self.tiles.push(Tile::Water);
		}
	}
}