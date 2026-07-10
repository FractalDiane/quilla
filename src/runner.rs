use std::env::args;
use std::io::Cursor;
use std::fs::read;

use bson::Document;

use quilla::quilla_story::QuillaStory;

fn main() {
	let filename = args().nth(1).unwrap();
	//let filename = "test_b.bson";
	let file_bytes = read(filename).unwrap();
	let cursor = Cursor::new(file_bytes);
	
	let story_doc = Document::from_reader(cursor).unwrap();
	let mut story = QuillaStory::new(&story_doc);
	println!("{}", story.continue_story());
	println!("{}", story.continue_story());
	println!("{}", story.continue_story());
	println!("{}", story.continue_story());
	println!("{}", story.continue_story());
	println!("{:?}", story.get_current_choices());
	
	story.select_choice(1);
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
}
