use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use crate::battle_objects::buildables::Wall;
use crate::battle_objects::coordinates::GridCoord;

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
		if loop_iterations > 250 {
			//println!("Pathing loop exceeded 250 iterations. Assuming no path and exiting.");
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
	println!("Exhausted open set without finding a path");
	None
}