/*
**	I really don't know what I'm doing.
**	- Dex 10/21/2022
*/

use std::env;
use std::num::Wrapping;
use bimap::*;

// Used if args left empty
const HELLO_WORLD_PROGRAM: &str =
"++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

fn main() {
	// Get arguments
	let args: Vec<String>	= env::args().collect();
	let programstr: &str	= if args.len() > 1 { &(args[1])[..] } else { HELLO_WORLD_PROGRAM };
	let program: Vec<char>	= programstr.chars().collect();

	let mut loop_cache: BiHashMap<usize, usize> = BiMap::new();


	// Index of pointer
	let mut ptr_i:	Wrapping<u8>	= Wrapping(0);
	let mut step:	usize			= 0;

	let mut memory: Vec<Wrapping<u8>> = vec![Wrapping(0); 256];

	
	while step < program.len() {
		let mut next_step: usize = step + 1;

		match program[step] {
			// Move pointer
			'>' => ptr_i += 1,
			'<' => ptr_i -= 1,

			// Modify cell
			'+' => memory[(ptr_i).0 as usize] += 1,
			'-' => memory[(ptr_i).0 as usize] -= 1,

			// Print Cell -> ASCII Char
			'.' => {
				let asciiv: u8 = memory[ptr_i.0 as usize].0;
				print!("{}", asciiv as char);
			},

			// Read ASCII Char -> Cell
			',' => (),





			// thank you so much http://brainfuck.org/brainfuck.html
			// for helping my brain not explode while coding this part
			'[' => {
				// If ptr == 0 skip past ]
				if (memory[(ptr_i).0 as usize]).0 == 0 {
					next_step = next_step;
				}
			},

			']' => {
				// If ptr != 0 skip back to [
				next_step = next_step;
			},

			
			
			_ => ()
		}

		step = next_step
	}
}