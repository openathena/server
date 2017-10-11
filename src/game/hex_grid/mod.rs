
pub mod offset;
pub mod coordinate;

pub use self::coordinate::*;

use std::convert::From;
use std::ops::{Index, IndexMut, Add};
use std::collections::HashMap;


pub struct HexGrid<T> {
	grid: HashMap<Coordinate, T>
}

impl<T> HexGrid<T> {
	pub fn new() -> HexGrid<T> {
		HexGrid {
			grid: HashMap::new()
		}
	}
	pub fn get<C: Into<Coordinate>>(&self, coord: C) -> Option<&T> {
		self.grid.get(&coord.into())
	}
	pub fn set<C: Into<Coordinate>>(&mut self, coord: C, value: T) {
		self.grid.insert(coord.into(), value);
	}
}