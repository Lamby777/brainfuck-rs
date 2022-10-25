/*
**	I really don't know what I'm doing.
**	- Dex 10/21/2022
*/

use std::env;
use std::num::Wrapping;
use bimap::*;
use text_io::read;

type LoopBounds = BiHashMap<usize, usize>;

const MEM_CELLS: usize = 30_000;

// Used if args left empty
const HELLO_WORLD_PROGRAM: &str =
"++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.";
//",>,[<+>-]<."; // Add inputs

fn main() {
	// Get arguments
	let args: Vec<String>			= env::args().collect();
	let programstr: &str			= if args.len() > 1 {	&(args[1])[..]
									} else {				HELLO_WORLD_PROGRAM };
	
	let program: Vec<u8>			= programstr.as_bytes().to_vec();

	let loops_precomp: LoopBounds = calc_loops(&programstr);

	let mut loop_stack: Vec<usize>	= vec![];


	// Index of pointer
	let mut ptr_i:	usize			= 0;
	let mut step:	usize			= 0;

	let mut memory: Vec<Wrapping<u8>> = vec![Wrapping(0); MEM_CELLS];

	
	while step < program.len() {
		let mut next_step: usize = step + 1;

		match program[step] {
			// Move pointer
			b'>' => {
				if ptr_i < MEM_CELLS { ptr_i += 1; }
				else { panic!("Attempt to overflow pointer location!"); }
			},
			b'<' => {
				if ptr_i > 0 { ptr_i -= 1; }
				else { panic!("Attempt to underflow pointer location!"); }
			},

			// Modify cell
			b'+' => memory[ptr_i] += 1,
			b'-' => memory[ptr_i] -= 1,

			// Print Cell -> ASCII Char
			b'.' => {
				let asciiv: u8 = memory[ptr_i].0;
				print!("{}", asciiv as char);
			},

			// Read ASCII Char -> Cell
			b',' => {
				memory[ptr_i] = Wrapping(getch());
			},


			// thank you so much http://brainfuck.org/brainfuck.html
			// for helping my brain not explode while coding this part
			b'[' => {
				// If ptr == 0 skip past ]
				if (memory[ptr_i]).0 == 0 {
					// Rest of program, after the current step
					next_step = *loops_precomp.get_by_left(&step).unwrap();
				} else {
					loop_stack.push(step);
				}
			},

			b']' => {
				// If ptr != 0 skip back to [
				if (memory[ptr_i]).0 != 0 {
					next_step = *loop_stack.last().unwrap() + 1;
				} else {
					loop_stack.pop();
				}
			},

			
			_ => ()
		}

		step = next_step;
	}
}

fn getch() -> u8 {
	let input: char = read!();
	input as u8
}

fn get_next_closing_bracket(slice: &str) -> usize {
	let chars: Vec<u8> = slice.as_bytes().to_vec();

	let mut ct: usize = 1;

	for i in 0..chars.len() {
		match chars[i] {
			b'['	=> { ct += 1 },
			b']'	=> { ct -= 1 },
			_		=> ()
		};

		if ct == 0 {
			return i;
		}
	};

	panic!("Loop Syntax Error");
}

fn calc_loops(tape: &str) -> LoopBounds {
	let mut res: LoopBounds = BiMap::<usize, usize>::new();

	// Find every [ and its matching ]
	for i in tape.match_indices("[") {
		let l_start = i.0;
		let rest: &str = &tape[l_start+1..];
		let l_end = get_next_closing_bracket(rest);

		res.insert(l_start, l_end);
	}

	res
}
