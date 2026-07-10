use std::env::args;
use std::io::Cursor;
use std::fs::read;

use evalexpr::error::EvalexprResultValue;
use quilla::quilla_story::{QuillaStory, CompiledStory};

fn main() {
	let filename = args().nth(1).unwrap();
	//let filename = "test.bson";
	let file_bytes = read(filename).unwrap();
	let cursor = Cursor::new(file_bytes);
	let story_data = bson::deserialize_from_reader::<_, CompiledStory>(cursor).unwrap();
	let mut story = QuillaStory::new(story_data.story);

	let mut testy = 5;

	story.register_function("double", evalexpr::Function::new(|num| {
		if let Ok(n) = num.as_number() {
			Ok(evalexpr::Value::Float(n * 2.0))
		} else {
			Err(evalexpr::EvalexprError::AppendedToLeafNode)
		}
	})).unwrap();

	//c(&evalexpr::Value::Empty);

	/*story.register_function("testy", evalexpr::Function::new(|_| {
		testy = 6;
		Ok(evalexpr::Value::Empty)
	})).unwrap();*/

	println!("{}", story.continue_story());
	println!("{}", story.continue_story());
	println!("{}", story.continue_story());
	println!("{}", story.continue_story());
	println!("{}", story.continue_story());
	println!("{:?}", story.get_current_choices());
	
	story.select_choice(2);
	println!("{}", story.continue_story());
	println!("{}", story.continue_story());
	println!("{}", story.continue_story());
	println!("{:?}", story.get_current_choices());

	story.select_choice(1);
	println!("{}", story.continue_story());
	println!("{}", story.continue_story());
	println!("{}", story.continue_story());
	println!("{}", story.continue_story());
	println!("{}", story.continue_story());
	println!("{}", story.continue_story());

	println!("{}", story.get_variable("testyer").unwrap());
	println!("{}", testy);
}
