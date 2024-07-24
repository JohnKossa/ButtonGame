pub fn line_to_line_intersect(first: ((i32, i32),(i32, i32)), second: ((i32, i32),(i32, i32))) -> bool {
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