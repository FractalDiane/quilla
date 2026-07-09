use std::env::args;
use std::io::Cursor;
use std::fs::read;

use bson::Document;

mod quilla_story;
mod variant;
use crate::quilla_story::QuillaStory;

fn main() {
	//let filename = args().nth(1).unwrap();
	let filename = "test_b.bson";
	let file_bytes = read(filename).unwrap();
	let cursor = Cursor::new(file_bytes);
	
	let mut story_doc = Document::from_reader(cursor).unwrap();
	let story = story_doc.get_array_mut("data").unwrap();

	let mut story = QuillaStory::new(story);
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
