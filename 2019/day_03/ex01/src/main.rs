use std::io::Read;
use std::io::stdin;

fn meets_criterion(x: u32) -> bool {
	let mut x = x;
	let mut prev_digit: Option<u32> = None;

	let mut is_decreasing_or_equal = true;
	let mut has_double = false;
	let mut prev_has_double = false;
	let mut prev_prev_has_double= false;
	let mut has_strict_double = false;

	while x != 0 {
		let digit = x % 10;

		match prev_digit {
			Some(prev_d) => {
				prev_prev_has_double = prev_has_double;
				prev_has_double = has_double;
				has_double = prev_d == digit;
				has_strict_double = has_strict_double || (prev_has_double && !has_double && !prev_prev_has_double);

				is_decreasing_or_equal = is_decreasing_or_equal && (prev_d >= digit);
			},
			None => {}
		}

		prev_digit = Some(digit);
		x = x / 10;
	}
	has_strict_double = has_strict_double || (!prev_has_double && has_double);

	is_decreasing_or_equal && has_strict_double
}

fn main() {
	let mut input = String::new();

	stdin().read_to_string(&mut input).unwrap();
	let lines: Vec<&str> = input.split("\n").collect::<Vec<&str>>();

	let range = lines[0].split("-").map(|x| {
		x.parse::<u32>().unwrap()
	}).collect::<Vec<u32>>();
	let range: (u32, u32) = (range[0].max(100000), range[1].min(999999));
	let mut result: u32= 0;

	for x in range.0..range.1 {
		if meets_criterion(x) {
			result += 1;
		}
	}

	println!("{}", result)
}
