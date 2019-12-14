use std::io::Read;
use std::io::stdin;
use std::collections::HashMap;

fn create_planets_map<'a>(input: &'a Vec<Vec<&str>>) -> HashMap<&'a str, Vec<&'a str>> {
	let mut orbits_of: HashMap<&str, Vec<&str>> = HashMap::new();

	for x in input {
		if !orbits_of.contains_key(x[0]) {
			orbits_of.insert(x[0], vec![]);
		}
		let mut orbiting_planets = orbits_of[x[0]].clone();
		orbiting_planets.push(x[1]);
		orbits_of.insert(x[0], orbiting_planets);
	}

	orbits_of
}

fn count_total_orbits(planets_map: & HashMap<&str, Vec<&str>>, root: &str) -> usize {
	let mut total_orbits: usize = 0;
	if let Some(root_orbiting_planets) = planets_map.get(root) {
		for orbiting_planet in root_orbiting_planets {
			total_orbits += 1;
			total_orbits += count_total_orbits(planets_map, orbiting_planet);
		}
	}

	total_orbits
}

fn main() {
	let mut input = String::new();

	stdin().read_to_string(&mut input).unwrap();
	let lines = input.split("\n").map(|x|{
		x.split(")").collect::<Vec<&str>>()
	}).collect::<Vec<Vec<&str>>>();

	let planets_map = create_planets_map(&lines);
	let mut indirect_orbits= 0;
	for x in planets_map.keys() {
		let current_indirect_orbits: usize = count_total_orbits(&planets_map, x);

		indirect_orbits += current_indirect_orbits;
	}

	println!("indirect orbits: {}", indirect_orbits);
}
