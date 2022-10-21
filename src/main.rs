/*
**	I really don't know what I'm doing.
**	- Dex 10/21/2022
*/

use std::env;

// Used if args left empty
const HELLO_WORLD_PROGRAM: &str =
	"++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

fn main() {
	// Get arguments
	let args: Vec<String> = env::args().collect();
	let program = if args.len() > 1 { &(args[1])[..] } else { HELLO_WORLD_PROGRAM };
}