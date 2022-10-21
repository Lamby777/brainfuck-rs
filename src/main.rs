/*
**	I really don't know what I'm doing.
**	- Dex 10/21/2022
*/

use std::env;
use std::num::Wrapping;

// Used if args left empty
const HELLO_WORLD_PROGRAM: &str =
"++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

fn main() {
	// Get arguments
	let args: Vec<String>	= env::args().collect();
	let programstr			= if args.len() > 1 { &(args[1])[..] } else { HELLO_WORLD_PROGRAM };
	let program: Vec<char>	= programstr.chars().collect();

	// Index of pointer
	let mut ptr_i:	Wrapping<u8>	= Wrapping(0);
	let mut step:	usize			= 0;

	let mut memory: Vec<Wrapping<u8>> = vec![Wrapping(0);256];

	
	while step < program.len() {
		let mut next_step = step + 1;

		match program[step] {
			'>' => ptr_i += 1,
			'<' => ptr_i -= 1,
			'+' => memory[(ptr_i).0 as usize] += 1,
			'-' => memory[(ptr_i).0 as usize] -= 1,
			'.' => {
				let cell_value = memory[ptr_i.0 as usize].0;
				//let ch = char::from_u32(cell_value as u32, 10).unwrap();
				let ch = cell_value as char;
				print!("{}", ch)
			},
			',' => (),
			'[' => {next_step = next_step},
			']' => {next_step = next_step;},
			_ => ()
		}

		step = next_step
	}
}