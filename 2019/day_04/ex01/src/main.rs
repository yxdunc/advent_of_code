use std::io::{stdout, Write, stdin};

fn add(op_index: usize, memory: &mut Vec<i32>, read_param: (i32, i32, i32)) -> usize {
	let p1 = read_param.0;
	let p2 = read_param.1;
	let p_res = memory[op_index + 3] as usize;

	memory[p_res] = p1 + p2;

	op_index + 4
}

fn mul(op_index: usize, memory: &mut Vec<i32>, read_param: (i32, i32, i32)) -> usize {
	let p1 = read_param.0;
	let p2 = read_param.1;
	let p_res = memory[op_index + 3] as usize;

	memory[p_res] = p1 * p2;

	op_index + 4
}

fn input(op_index: usize, memory: &mut Vec<i32>) -> usize {
	let mut s=String::new();
	print!("ID of the system to test: ");
	stdout().flush();
	stdin().read_line(&mut s).expect("Something went wrong...");
	let write_index = memory[op_index + 1] as usize;
	memory[write_index] = s.trim().parse::<i32>().expect("Input wasn't an int");

	op_index + 2
}

fn output(op_index: usize, memory: &mut Vec<i32>, read_param: (i32, i32, i32)) -> usize {
	println!("> {}", read_param.0);

	op_index + 2
}

fn jump_if_true(op_index: usize, memory: &mut Vec<i32>, read_param: (i32, i32, i32)) -> usize {
	if read_param.0 != 0 {
		return read_param.1 as usize
	} else {
		return op_index + 3
	}
}

fn jump_if_false(op_index: usize, memory: &mut Vec<i32>, read_param: (i32, i32, i32)) -> usize {
	if read_param.0 == 0 {
		return read_param.1 as usize
	} else {
		return op_index + 3
	}
}

fn less_than(op_index: usize, memory: &mut Vec<i32>, read_param: (i32, i32, i32)) -> usize {
	let p_res = memory[op_index + 3] as usize;

	if read_param.0 < read_param.1 {
		memory[p_res] = 1;
	} else {
		memory[p_res] = 0;
	}
	op_index + 4
}

fn equal_to(op_index: usize, memory: &mut Vec<i32>, read_param: (i32, i32, i32)) -> usize {
	let p_res = memory[op_index + 3] as usize;

	if read_param.0 == read_param.1 {
		memory[p_res] = 1;
	} else {
		memory[p_res] = 0;
	}
	op_index + 4
}

fn split_op_code(op_code: usize) -> (usize, usize, usize, usize) {
	(
		op_code % 100,
		op_code / 100 % 10,
		op_code / 1000 % 10,
		op_code / 10000 % 10,
	)
}

fn get_param_value(index: usize, memory: &mut Vec<i32>, param_type: usize) -> i32 {
	match param_type {
		0 => {
			if memory.len() > index && memory.len() > memory[index] as usize {
				memory[memory[index] as usize]
			} else {
				0
			}
		},
		1 => {
			if memory.len() > index {
				memory[index]
			} else {
				0
			}
		},
		_ => panic!("Something went wrong... index: {}, param_type: {}", index, param_type)
	}
}

fn get_read_param(index: usize, memory: &mut Vec<i32>, op_code: (usize, usize, usize, usize)) -> (i32, i32, i32) {
	(
		get_param_value(index + 1, memory, op_code.1),
		get_param_value(index + 2, memory, op_code.2),
		get_param_value(index + 3, memory, op_code.3),
	)
}

fn start_computer(memory: &mut Vec<i32>) {
	let mut index: usize = 0;

	loop {
		let op_code = split_op_code(memory[index] as usize);
		let read_param= get_read_param(index, memory, op_code);

		index = match op_code.0 {
			1 => add(index, memory, read_param),
			2 => mul(index, memory, read_param),
			3 => input(index, memory),
			4 => output(index, memory, read_param),
			5 => jump_if_true(index, memory, read_param),
			6 => jump_if_false(index, memory, read_param),
			7 => less_than(index, memory, read_param),
			8 => equal_to(index, memory, read_param),
			99 => break,
			_ => panic!("Something went wrong... index {}", index)
		}
	}
}

fn main() {
	let mut memory: Vec<i32> = vec![3,225,1,225,6,6,1100,1,238,225,104,0,1101,40,71,224,1001,224,-111,224,4,224,1002,223,8,223,101,7,224,224,1,224,223,223,1102,66,6,225,1102,22,54,225,1,65,35,224,1001,224,-86,224,4,224,102,8,223,223,101,6,224,224,1,224,223,223,1102,20,80,225,101,92,148,224,101,-162,224,224,4,224,1002,223,8,223,101,5,224,224,1,224,223,223,1102,63,60,225,1101,32,48,225,2,173,95,224,1001,224,-448,224,4,224,102,8,223,223,1001,224,4,224,1,224,223,223,1001,91,16,224,101,-79,224,224,4,224,1002,223,8,223,101,3,224,224,1,224,223,223,1101,13,29,225,1101,71,70,225,1002,39,56,224,1001,224,-1232,224,4,224,102,8,223,223,101,4,224,224,1,223,224,223,1101,14,59,225,102,38,143,224,1001,224,-494,224,4,224,102,8,223,223,101,3,224,224,1,224,223,223,1102,30,28,224,1001,224,-840,224,4,224,1002,223,8,223,101,4,224,224,1,223,224,223,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,107,677,226,224,1002,223,2,223,1005,224,329,1001,223,1,223,8,226,226,224,102,2,223,223,1006,224,344,101,1,223,223,7,226,677,224,1002,223,2,223,1005,224,359,101,1,223,223,1007,677,226,224,1002,223,2,223,1005,224,374,1001,223,1,223,1007,677,677,224,1002,223,2,223,1006,224,389,101,1,223,223,1008,226,226,224,1002,223,2,223,1005,224,404,1001,223,1,223,108,677,226,224,1002,223,2,223,1006,224,419,1001,223,1,223,1108,677,226,224,102,2,223,223,1006,224,434,1001,223,1,223,108,226,226,224,1002,223,2,223,1005,224,449,101,1,223,223,7,677,677,224,1002,223,2,223,1006,224,464,1001,223,1,223,8,226,677,224,1002,223,2,223,1005,224,479,1001,223,1,223,107,226,226,224,102,2,223,223,1006,224,494,101,1,223,223,1007,226,226,224,1002,223,2,223,1005,224,509,1001,223,1,223,1107,226,677,224,102,2,223,223,1005,224,524,1001,223,1,223,108,677,677,224,1002,223,2,223,1005,224,539,101,1,223,223,1107,677,226,224,102,2,223,223,1005,224,554,1001,223,1,223,107,677,677,224,1002,223,2,223,1005,224,569,101,1,223,223,8,677,226,224,102,2,223,223,1005,224,584,1001,223,1,223,7,677,226,224,102,2,223,223,1006,224,599,101,1,223,223,1008,677,677,224,1002,223,2,223,1005,224,614,101,1,223,223,1008,677,226,224,102,2,223,223,1006,224,629,1001,223,1,223,1108,677,677,224,102,2,223,223,1006,224,644,101,1,223,223,1108,226,677,224,1002,223,2,223,1005,224,659,1001,223,1,223,1107,226,226,224,102,2,223,223,1006,224,674,1001,223,1,223,4,223,99,226];
//	let mut memory: Vec<i32> = vec![3,9,8,9,10,9,4,9,99,-1,8]; // input equal 8
//	let mut memory: Vec<i32> = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99]; // cmd input to 8 lt=999 eq=1000 gt=1001
	start_computer(&mut memory);
}
