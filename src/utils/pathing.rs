use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use crate::battle_objects::buildables::Wall;
use crate::battle_objects::coordinates::GridCoord;

/*pub fn bfs_pathing(from: GridCoord, to:GridCoord, walls: HashSet<Wall>, blocked_squares: HashSet<GridCoord>)-> Option<Vec<GridCoord>>{
	//walls prevent two grid coords from being connected
	//blocked squares completely block a grid coord from all sides
	//use A* to find the shortest path if a path exist
	//return None if no path exists
	//return Some(Vec<GridCoord>) if a path exists
	//return Some(Vec::new()) if from == to
	let connected_squares = |square: GridCoord| -> Vec<GridCoord>{
		let mut connected = vec![square.to_north(1),square.to_south(1),square.to_west(1),square.to_east(1)];
		connected.filter(|&coord| !blocked_squares.contains(&coord) && !walls.iter().any(|wall| wall.is_blocking(square, coord))).collect()
	};
	let mut available_squares = Vec::new();
	connected_squares(from).iter().for_each(|&coord|{available_squares.push(coord);});
	let mut path = Vec::new();
	while !available_squares.is_empty(){
		let current = available_squares.remove(0);
		if current == to{
			path.append(current);
			return Some(path);

			let mut path = Vec::new();
			let mut current_pos = current;
			while current_pos != from{
				path.push(current_pos);
				current_pos = came_from[&current_pos];
			}
			path.push(from);
			path.reverse();
			return Some(path);
		}

	}
	None
}*/

#[derive(Clone, Copy)]
struct PathingNode{
	position: GridCoord,
	cost: f32,
	priority: f32,
}

impl PartialEq for PathingNode {
	fn eq(&self, other: &Self) -> bool {
		self.priority == other.priority
	}
}

impl Eq for PathingNode {}

impl PartialOrd<Self> for PathingNode {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for PathingNode{
	fn cmp(&self, other: &Self) -> Ordering {
		other.priority.partial_cmp(&self.priority).unwrap_or(Ordering::Equal)
	}
}

pub fn path_to(
	from: GridCoord,
	to: GridCoord,
	walls: &Vec<Wall>,
	blocked_squares: &Vec<GridCoord>,
) -> Option<Vec<GridCoord>> {
	let mut loop_iterations = 0;
	if from == to {
		return Some(Vec::new());
	}

	let mut open_set = BinaryHeap::new();
	let mut came_from = HashMap::new();
	let mut g_score = HashMap::new();
	let mut f_score = HashMap::new();

	open_set.push(PathingNode { position: from, cost: 0.0, priority: from.pythagorean_distance_to(&to) });
	g_score.insert(from, 0.0);
	f_score.insert(from, from.pythagorean_distance_to(&to));

	let connected_squares = |square: GridCoord| -> Vec<GridCoord> {
		let mut connected = Vec::new();
		let directions = [square.to_north(1), square.to_south(1), square.to_west(1), square.to_east(1)];
		for &dir in directions.iter() {
			if !blocked_squares.contains(&dir) && !walls.iter().any(|wall| wall.is_blocking(square, dir)) {
				connected.push(dir);
			}
		}
		connected
	};

	while let Some(current) = open_set.pop() {
		loop_iterations += 1;
		// if loop_iterations > 1000 {
		// 	println!("Pathing loop exceeded 1000 iterations");
		// 	println!("Open set size is {}", open_set.len());
		// 	//return None;
		// }
		if loop_iterations > 5000{
			println!("Pathing loop exceeded 5000 iterations");
			println!("Open set size is {}", open_set.len());
			return None;
		}
		if current.position == to {
			let mut path = Vec::new();
			let mut current_pos = current.position;
			while let Some(&prev) = came_from.get(&current_pos) {
				path.push(current_pos);
				current_pos = prev;
			}
			//path.push(from);
			path.reverse();
			println!("Path found");
			return Some(path);
		}

		for neighbor in connected_squares(current.position) {
			let tentative_g_score = g_score[&current.position] + 1.0;
			if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&f32::MAX) {
				came_from.insert(neighbor, current.position);
				g_score.insert(neighbor, tentative_g_score);
				let f_score_neighbor = tentative_g_score + neighbor.pythagorean_distance_to(&to);
				f_score.insert(neighbor, f_score_neighbor);
				open_set.push(PathingNode { position: neighbor, cost: tentative_g_score, priority: f_score_neighbor });
			}
		}
	}
	println!("Exhaused open set without finding a path");
	None
}