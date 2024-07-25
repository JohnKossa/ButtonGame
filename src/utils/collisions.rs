
type Point = (i32, i32);
type LineSegment = (Point, Point); //start, end
type Square = (Point,u32); //center, side length

pub fn line_to_line_intersect(first: LineSegment, second: LineSegment) -> bool {
	//https://stackoverflow.com/questions/563198/how-do-you-detect-where-two-line-segments-intersect/565282#565282

	let cross = |a: (f32, f32), b: (f32, f32)| -> f32 {
		a.0 * b.1 - a.1 * b.0
	};
	let minus = |a: (f32, f32), b: (f32, f32)| -> (f32, f32) {
		(a.0 - b.0, a.1 - b.1)
	};
	let div = |a: (f32, f32), b: f32| -> (f32, f32) {
		(a.0 / b, a.1 / b)
	};
	let to_f32 = |a: (i32, i32)| -> (f32, f32) {
		(a.0 as f32, a.1 as f32)
	};
	let interval_of_intersection = (0.0, 1.0);
	let p:(f32, f32) = to_f32(first.0);
	let q:(f32, f32) = to_f32(second.0);
	let r:(f32, f32) = minus(to_f32(first.1), to_f32(first.0));
	let s:(f32, f32) = minus(to_f32(second.1), to_f32(second.0));

	let r_cross_s = cross(r, s);
	let q_minus_p = (q.0 - p.0, q.1 - p.1);
	let q_minus_p_cross_r = q_minus_p.0 * r.1 - q_minus_p.1 * r.0;

	if r_cross_s == 0.0 {
		if q_minus_p_cross_r == 0.0 {
			true //collinear
		} else {
			false //parallel
		}
	} else {
		let t = cross(q_minus_p, div(s, r_cross_s));
		let u = cross(q_minus_p, div(r, r_cross_s));
		//are intersecion coordinates in range?
		let t_in_range = interval_of_intersection.0 <= t && t <= interval_of_intersection.1;
		let u_in_range = interval_of_intersection.0 <= u && u <= interval_of_intersection.1;
		if t_in_range && u_in_range {
			true //intersecting
		} else {
			false //disjoint
		}
	}
}

pub fn line_to_square_intersect(line: LineSegment, square: Square) -> bool {
	let point_in_square = |point: (i32, i32), square: Square| -> bool {
		let (center, side) = square;
		let (x, y) = point;
		let (center_x, center_y) = center;
		let half_side = side as f32 / 2.0;
		if x >= (center_x as f32 - half_side) as i32 && x <= (center_x as f32 + half_side) as i32 && y >= (center_y as f32 - half_side) as i32 && y <= (center_y as f32 + half_side) as i32 {
			true
		} else {
			false
		}
	};
	let square_left_side = |square: Square| -> LineSegment {
		let (center, side) = square;
		let (center_x, center_y) = center;
		let half_side = side as f32 / 2.0;
		((center_x - half_side as i32, center_y - half_side as i32), (center_x - half_side as i32, center_y + half_side as i32))
	};
	let square_top_side = |square: Square| -> LineSegment {
		let (center, side) = square;
		let (center_x, center_y) = center;
		let half_side = side as f32 / 2.0;
		((center_x - half_side as i32, center_y - half_side as i32), (center_x + half_side as i32, center_y - half_side as i32))
	};
	let square_right_side = |square: Square| -> LineSegment {
		let (center, side) = square;
		let (center_x, center_y) = center;
		let half_side = side as f32 / 2.0;
		((center_x + half_side as i32, center_y - half_side as i32), (center_x + half_side as i32, center_y + half_side as i32))
	};
	let square_bottom_side = |square: Square| -> LineSegment {
		let (center, side) = square;
		let (center_x, center_y) = center;
		let half_side = side as f32 / 2.0;
		((center_x - half_side as i32, center_y + half_side as i32), (center_x + half_side as i32, center_y + half_side as i32))
	};
	//if either endpoint of the line is inside the square, true
	if point_in_square(line.0, square) || point_in_square(line.1, square){
		return true;
	}
	//if both points are left of the square's left side, false
	if line.0.0 < square.0 .0 - square.1 as i32 / 2 && line.1.0 < square.0 .0 - square.1 as i32 / 2{
		return false;
	}
	//if both points are right of the square's right side, false
	if line.0.0 > square.0 .0 + square.1 as i32 / 2 && line.1.0 > square.0 .0 + square.1 as i32 / 2{
		return false;
	}
	//if both points are above the square's top side, false
	if line.0.1 < square.0 .1 - square.1 as i32 / 2 && line.1.1 < square.0 .1 - square.1 as i32 / 2{
		return false;
	}
	//if both points are below the square's bottom side, false
	if line.0.1 > square.0 .1 + square.1 as i32 / 2 && line.1.1 > square.0 .1 + square.1 as i32 / 2{
		return false;
	}
	//if the line intersects any of the square's sides, true
	if line_to_line_intersect(line, square_left_side(square)){
		return true;
	}
	if line_to_line_intersect(line, square_top_side(square)){
		return true;
	}
	if line_to_line_intersect(line, square_right_side(square)){
		return true;
	}
	if line_to_line_intersect(line, square_bottom_side(square)){
		return true;
	}
	false
}

