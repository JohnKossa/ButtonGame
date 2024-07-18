use std::f32::consts::PI;
use sdl2::rect::Point;

#[derive(Clone, Copy, Debug)]
pub struct GameCoord{
	pub x: i32,
	pub y: i32,
}
impl GameCoord {
	pub fn to_grid_coord(&self) -> GridCoord{
		let grid_size = GridCoord::grid_size();
		let mut x_grid = (self.x.abs() + grid_size/2) / grid_size;
		let mut y_grid = (self.y.abs() + grid_size/2) / grid_size;
		if self.x < 0 {
			x_grid = x_grid * -1
		}
		if self.y < 0 {
			y_grid = y_grid * -1
		}
		GridCoord{x: x_grid, y: y_grid }
	}

	pub fn to_display_coord(&self, center_point: GameCoord, scale_factor: f32, window_dimensions: (u32, u32)) -> sdl2::rect::Point{
		//translate center to 0,0
		//scale by the scale factor
		//translate back to w/2, h/2

		//translate point by same dx and dy
		//scale by scale factor,
		//translate back by adding w/2 h/2

		let mut new_x = self.x - center_point.x;
		new_x = (new_x as f32 * scale_factor) as i32;
		new_x = new_x + window_dimensions.0 as i32/2;

		let mut new_y = self.y - center_point.y;
		new_y = (new_y as f32 * scale_factor) as i32;
		new_y = new_y + window_dimensions.1 as i32/2;

		Point::new(new_x, new_y)
	}
}

impl PartialEq for GameCoord {
	fn eq(&self, other: &Self) -> bool {
		self.x == other.x && self.y == other.y
	}
}

#[derive(Clone, Copy, Debug)]
pub struct GridCoord{
	//pub grid_size: i32,
	pub x: i32,
	pub y: i32,
}

impl GridCoord {
	pub fn grid_size()->i32{
		20
	}

	pub fn top_left(&self) -> GameCoord {
		let center = self.center();
		GameCoord {x: center.x-Self::grid_size()/2, y: center.y-Self::grid_size()/2}
	}
	pub fn top_right(&self) -> GameCoord {
		let center = self.center();
		GameCoord {x: center.x+Self::grid_size()/2, y: center.y-Self::grid_size()/2}
	}
	pub fn bottom_left(&self) -> GameCoord {
		let center = self.center();
		GameCoord {x: center.x-Self::grid_size()/2, y: center.y+Self::grid_size()/2}
	}
	pub fn bottom_right(&self) -> GameCoord {
		let center = self.center();
		GameCoord {x: center.x+Self::grid_size()/2, y: center.y+Self::grid_size()/2}
	}
	pub fn center(&self) -> GameCoord {
		GameCoord {x: Self::grid_size()*self.x, y: Self::grid_size()*self.y}
	}
}

impl PartialEq for GridCoord {
	fn eq(&self, other: &Self) -> bool {
		self.x == other.x && self.y == other.y
	}
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum Direction {North, South, East, West}

impl Direction{
	pub(crate) fn from_facing_vector(facing_vector: f32) -> Direction{
		match facing_vector {
			x if x==0.0 => Direction::East,
			x if x ==0.5*PI => Direction::North,
			x if x==PI || x==-PI => Direction::West,
			x if x==-0.5*PI => Direction::South,
			x if x <= 0.25 *PI && x >= -0.25*PI => Direction::East,
			x if x < 0.75*PI && x > 0.25*PI => Direction::North,
			x if x > -0.75*PI && x < -0.25*PI => Direction::South,
			x if x > 1.25*PI && x < 1.75*PI => Direction::South,
			x if x >= 1.75*PI && x <= 2.25*PI => Direction::East,
			x if x >= 0.75*PI || x <= -0.75*PI => Direction::West,
			_ => {unreachable!("impossible value in facing vector")}
		}
	}
}