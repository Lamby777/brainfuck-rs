/*
**	I really don't know what I'm doing.
**	- Dex 10/21/2022
*/

use std::env;
use std::num::Wrapping;

const MEM_CELLS: usize = 30_000;

// Used if args left empty
const HELLO_WORLD_PROGRAM: &str =
"++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.";

fn main() {
	// Get arguments
	let args: Vec<String>			= env::args().collect();
	let programstr: &str			= if args.len() > 1 {	&(args[1])[..]
									} else {				HELLO_WORLD_PROGRAM };
	
	let program: Vec<char>			= programstr.chars().collect();

	let mut loop_stack: Vec<usize>	= vec![];


	// Index of pointer
	let mut ptr_i:	usize			= 0;
	let mut step:	usize			= 0;

	let mut memory: Vec<Wrapping<u8>> = vec![Wrapping(0); MEM_CELLS];

	
	while step < program.len() {
		let mut next_step: usize = step + 1;

		print!("\n{}\t| {}", step, program[step]);

		match program[step] {
			// Move pointer
			'>' => {
				if ptr_i < MEM_CELLS {
					ptr_i += 1
				} else {
					panic!("Attempt to overflow pointer location!");
				}
			},
			'<' => {
				if ptr_i > 0 {
					ptr_i -= 1
				} else {
					panic!("Attempt to underflow pointer location!");
				}
			},

			// Modify cell
			'+' => memory[ptr_i] += 1,
			'-' => memory[ptr_i] -= 1,

			// Print Cell -> ASCII Char
			'.' => {
				let asciiv: u8 = memory[ptr_i].0;
				print!(" {} ({})", asciiv, asciiv as char);
			},

			// Read ASCII Char -> Cell
			',' => (),





			// thank you so much http://brainfuck.org/brainfuck.html
			// for helping my brain not explode while coding this part
			'[' => {
				// If ptr == 0 skip past ]
				if (memory[ptr_i]).0 == 0 {
					// Rest of program, after the current step
					let rest: &str = &programstr[step+1..];
					next_step = step + get_next_closing_bracket(rest);
				} else {
					loop_stack.push(step);
				}
			},

			']' => {
				// If ptr != 0 skip back to [
				if ptr_i != 0 {
					next_step = *loop_stack.last().unwrap() + 1;
				} else {
					loop_stack.pop();
				}
				//dbg!(ptr_i.0, step, next_step);
			},

			
			
			_ => ()
		}

		step = next_step
	}
}

fn get_next_closing_bracket(slice: &str) -> usize {
	let chars: Vec<char> = slice.chars().collect();

	let mut ct: usize = 1;

	for i in 0..chars.len() {
		match chars[i] {
			'['	=> (ct += 1),
			']'	=> (ct -= 1),
			_	=> ()
		};

		if ct == 0 {
			return i;
		}
	};

	panic!("Loop Syntax Error");
}
