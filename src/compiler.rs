use std::env::args;

use quilla::quilla_compiler::compile_story_to_file;

fn main() {
	let filename = args().nth(1).unwrap();
	match compile_story_to_file(&filename) {
		Ok(_) => {
			println!("Compiled to {}", filename.replace(".quilla", ".bson"));
		},
		Err(err) => {
			eprintln!("Error compiling: {}", err);
		}
	}
}
