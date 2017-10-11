use super::hex_grid::*;

#[derive(Copy, Clone)]
pub struct Tile {
	tile_type: TileType
}

#[derive(Copy, Clone)]
pub enum TileType {
	Water,
	Land
}


pub struct World {
	/** distance from center to edge of map (Hexagon shape map) */
	radius: u16,
	grid: HexGrid<Tile>
}

impl World {
	pub fn new(radius: u16) -> World {
		let mut world = World {
			radius,
			grid: HexGrid::new()
		};
		world.init();
		world
	}

	fn init(&mut self) {
		let default_tile = Tile {
			tile_type: TileType::Water
		};
		(Offset::fill_hex(self.radius) + CENTER).iter().for_each(|coord| {
			self.grid.set(*coord, default_tile.clone())
		})
	}
}