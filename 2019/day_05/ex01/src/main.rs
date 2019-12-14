use std::io::Read;
use std::io::stdin;
use std::collections::HashMap;
use std::collections::HashSet;

struct Planet<'a> {
	visited: bool,
	orbiting_planets: HashSet<&'a str>
}

impl <'a>Planet<'a> {
	fn new(orbiting_planets: HashSet<& str>) -> Planet {
		Planet{
			visited: false,
			orbiting_planets
		}
	}
}

fn add_link<'a>(planet_map: &mut HashMap<&'a str, Planet<'a>>, planet_x: &'a str, planet_y: &'a str) {
	if !planet_map.contains_key(planet_x) {
		planet_map.insert(planet_x, Planet::new(HashSet::new()));
	}
	planet_map.get_mut(planet_x).unwrap().orbiting_planets.insert(planet_y);
}

fn create_planets_map<'a>(input: &'a Vec<Vec<&str>>) -> HashMap<&'a str, Planet<'a>> {
	let mut planet_map: HashMap<&str, Planet> = HashMap::new();

	for x in input {
		add_link(&mut planet_map, x[0], x[1]);
		add_link(&mut planet_map, x[1], x[0]);
	}

	planet_map
}

fn get_shortest_path_len(planets_map: &mut HashMap<&str, Planet>, planet_x: &str, planet_y: &str)
	-> (usize, bool) {
	let mut shortest_path_len: usize = std::usize::MAX;
	let planet_x_content = planets_map.get_mut(planet_x).expect("planet_not_found");
	let mut found_planet_y = false;

	if planet_x_content.visited {
		return (shortest_path_len, false);
	}

	planet_x_content.visited = true;

	for orbiting_planet in planet_x_content.orbiting_planets.clone() {
			if (*orbiting_planet).eq(planet_y){
				return (0, true);
			}
			let (sub_path_len, sub_found_planet_y) = get_shortest_path_len(
				planets_map,
				orbiting_planet,
				planet_y
			);

			if sub_found_planet_y {
				shortest_path_len = shortest_path_len.min(sub_path_len);
				found_planet_y = true;
			}
	}
	if found_planet_y {
		shortest_path_len += 1;
	}

	(shortest_path_len, found_planet_y)
}

fn main() {
	let mut input = String::new();

	stdin().read_to_string(&mut input).unwrap();
	let lines = input.split("\n").map(|x|{
		x.split(")").collect::<Vec<&str>>()
	}).collect::<Vec<Vec<&str>>>();

	let mut planets_map = create_planets_map(&lines);
	let (shortest_path_len, _) = get_shortest_path_len(
		&mut planets_map,
		"YOU",
		"SAN"
	);

	println!("shortest path to the planet that santa is orbiting: {}", shortest_path_len - 1);
}
