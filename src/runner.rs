use std::env::args;
use std::io::Cursor;
use std::fs::read;

use quilla::quilla_story::{QuillaStory, CompiledStory};

fn main() {
	let filename = args().nth(1).unwrap();
	//let filename = "test.bson";
	let file_bytes = read(filename).unwrap();
	let cursor = Cursor::new(file_bytes);
	let story_data = bson::deserialize_from_reader::<_, CompiledStory>(cursor).unwrap();
	let mut story = QuillaStory::new(story_data.story);

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

	story.select_choice(0);
	println!("{}", story.continue_story());
	println!("{}", story.continue_story());
	println!("{}", story.continue_story());
	println!("{}", story.continue_story());
	println!("{}", story.continue_story());
	println!("{}", story.continue_story());
}
