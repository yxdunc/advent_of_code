use std::io;
use	std::collections::HashSet;
use std::io::Read;

fn mark_linear_trail(trail: &mut HashSet<(i32, i32)>, pos: (i32, i32), dir: char, steps: i32) -> (i32, i32) {
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
		trail.insert(pos);
	}
	pos
}

fn mark_trail(trail: &mut HashSet<(i32, i32)>, wire_op: Vec<&str>) {
	let mut pos: (i32, i32) = (0, 0);

	for x in wire_op {
		if x.len() == 0 { break }
		let dir = x.as_bytes()[0] as char;
		let steps = x[1..].parse::<i32>().unwrap();
		pos = mark_linear_trail(trail, pos, dir, steps);
	}
}

fn get_intersections(wire_0: Vec<&str>, wire_1: Vec<&str>) -> Vec<(i32, i32)> {
	let mut pos: (i32, i32) = (0, 0);
	let mut trail_wire_0: HashSet<(i32, i32)> = HashSet::new();
	let mut trail_wire_1: HashSet<(i32, i32)> = HashSet::new();

//	println!("wire_0: {:?}", wire_0);
//	println!("wire_1: {:?}", wire_1);

	mark_trail(&mut trail_wire_0, wire_0);
	mark_trail(&mut trail_wire_1, wire_1);

//	println!("trail_wire_0: {:?}", trail_wire_0);
//	println!("trail_wire_1: {:?}", trail_wire_1);

	trail_wire_0.intersection(&trail_wire_1).cloned().collect()
}

fn get_closest_point(points: &Vec<(i32, i32)>) -> (i32, i32) {
	*points.into_iter().min_by(|a, b| {
        (a.0.abs() + a.1.abs()).cmp(&(b.0.abs() + b.1.abs()))
    }).unwrap()
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
	let intersections = get_intersections(wire_0, wire_1);
	let closest_point = get_closest_point(&intersections);

//	println!("intersections: {:?}", intersections);
//	println!("closest: {:?}", closest_point);
	println!("distance: {:?}", closest_point.0 + closest_point.1);
}
