use std::io;
use	std::collections::HashMap;
use std::io::Read;
use std::cmp::min;

fn mark_linear_trail(trail: &mut HashMap<(i32, i32), i32>, pos: (i32, i32), dir: char, steps: i32, wire_curr_len: &mut i32) -> (i32, i32) {
	let mut pos= pos;
	let dir: (i32,i32) = match dir {
		'D' => { (0, -1) }
		'U' => { (0, 1) }
		'L' => { (-1, 0) }
		'R' => { (1, 0) }
		_ => { println!("Something's wrong {}", dir); (0, 0) }
	};

	for x in 0..steps {
		pos = (pos.0 + dir.0, pos.1 + dir.1);
		*wire_curr_len += 1;
		if !trail.contains_key(&pos) {
			trail.insert(pos, *wire_curr_len);
		}
	}
	pos
}

fn mark_trail(trail: &mut HashMap<(i32, i32), i32>, wire_op: Vec<&str>) {
	let mut pos: (i32, i32) = (0, 0);
	let mut curr_wire_len: i32 = 0;

	for x in wire_op {
		if x.len() == 0 { break }
		let dir = x.as_bytes()[0] as char;
		let steps = x[1..].parse::<i32>().unwrap();
		pos = mark_linear_trail(trail, pos, dir, steps, &mut curr_wire_len);
	}
}

fn intersection_distances(trail_0: & HashMap<(i32, i32), i32>, trail_1: & HashMap<(i32, i32), i32>) -> Vec<i32>{
	let mut results: Vec<i32> = vec![];
	for (k, v) in trail_0 {
		if trail_1.contains_key(k) { results.push(*v + *trail_1.get(k).unwrap() ) }
	}

	results
}

fn get_intersections_distances(wire_0: Vec<&str>, wire_1: Vec<&str>) -> Vec<(i32)> {
	let mut pos: (i32, i32) = (0, 0);
	let mut trail_wire_0: HashMap<(i32, i32), i32> = HashMap::new();
	let mut trail_wire_1: HashMap<(i32, i32), i32> = HashMap::new();

	mark_trail(&mut trail_wire_0, wire_0);
	mark_trail(&mut trail_wire_1, wire_1);

	intersection_distances(&trail_wire_0, &trail_wire_1)
}

fn main() {
	let mut input = String::new();

	io::stdin().read_to_string(&mut input).unwrap();
	let wires: Vec<&str> = input.split("\n").collect::<Vec<&str>>();

	let wire_0 = wires[0].clone()
		.split(",")
		.collect::<Vec<&str>>();
	let wire_1 = wires[1].clone()
		.split(",")
		.collect::<Vec<&str>>();
	let intersections = get_intersections_distances(wire_0, wire_1);
	let closest_point_distance = intersections.into_iter().min();

	println!("distance: {:?}", closest_point_distance.unwrap());
}
