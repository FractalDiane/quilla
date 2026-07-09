use std::env::args;
use std::io::{BufRead, BufReader, Write};
use std::fs::File;

use bson::{Bson, Document, doc};

mod quilla_story;
mod variant;

enum ContainerEntry {
	Choice(Vec<(String, Vec<Document>)>),
	If(Vec<(String, Vec<Document>)>, Vec<Document>),
}

impl ContainerEntry {
	pub fn add_entry(&mut self, entry: (String, Vec<Document>)) {
		match self {
			ContainerEntry::Choice(choices) => {
				choices.push(entry);
			},
			ContainerEntry::If(branches, _) => {
				branches.push(entry);
			},
		}
	}

	/*pub fn add_item(&mut self, item: Document) {
		match self {
			ContainerEntry::Choice(choices) => {
				choices.last_mut().unwrap().1.push(item);
			},
			ContainerEntry::If(branches, _) => {
				branches.last_mut().unwrap().1.push(item);
			},
		}
	}*/

	pub fn get_item_array(&mut self) -> &mut Vec<Document> {
		match self {
			ContainerEntry::Choice(choices) => {
				&mut choices.last_mut().unwrap().1
			},
			ContainerEntry::If(branches, _) => {
				&mut branches.last_mut().unwrap().1
			},
		}
	}
}

fn main() {
	let filename = args().nth(1).unwrap();
	//let filename = "test_nesting.quilla";
	let file = BufReader::new(File::open(&filename).unwrap());

	let mut story = Vec::<Document>::new();
	//let mut choices_stack = Vec::<Vec<(String, Vec<Document>)>>::new();
	let mut choices_stack = Vec::<ContainerEntry>::new();

	for line_result in file.lines() {
		let line = line_result.unwrap();
		let line_split = line.split_whitespace().collect::<Vec<&str>>();
		if line_split.is_empty() {
			continue;
		}

		let indent_level = line.chars().position(|ch| ch != '\t').unwrap_or(0);
		if indent_level < choices_stack.len() {
			let next_is_choice = line_split[0].starts_with('+');
			if !next_is_choice || line_split[0].len() < choices_stack.len() {
				for _ in indent_level + next_is_choice as usize..choices_stack.len()  {
					let choices_to_add = choices_stack.pop().unwrap();
					let target_array = if choices_stack.is_empty() {
						&mut story
					} else {
						choices_stack.last_mut().unwrap().get_item_array()
					};

					match choices_to_add {
						ContainerEntry::Choice(choices) => {
							target_array.push(doc!{
								"type": "choice",
								"choices": choices.iter().map(|ch| ch.0.clone()).collect::<Vec<String>>(),
								"results": choices.iter().map(|ch| ch.1.clone()).collect::<Vec<Vec<Document>>>(),
							});
						},
						ContainerEntry::If(branches, _) => {
							target_array.push(doc!{
								"type": "if",
								"conditions": branches.iter().map(|ch| ch.0.clone()).collect::<Vec<String>>(),
								"branches": branches.iter().map(|ch| ch.1.clone()).collect::<Vec<Vec<Document>>>(),
							});
						},
					}
				}
			}
		}

		let target_array = if choices_stack.is_empty() {
			&mut story
		} else {
			//&mut choices_stack.last_mut().unwrap().last_mut().unwrap().1
			choices_stack.last_mut().unwrap().get_item_array()
		};

		if line_split[0].starts_with('@') {
			let keyword = &line_split[0][1..];
			match keyword {
				"VAR" => {
					target_array.push(doc!{
						"type": "var",
						"name": line_split[1],
						"value": line_split[3],
					});
				},
				"SET" => {
					target_array.push(doc!{
						"type": "set",
						"name": line_split[1],
						"value": line_split[3],
					});
				},
				"IF" => {
					target_array.push(doc!{
						"type": "if",
						"condition": line_split[1..].join(" "),
					});
					//let condition = line_split[1..].join(" ");
				},
				_ => {

				},
			}
		} else if line_split[0].starts_with('+') {
			let choice = line_split[1..].join(" ");
			let depth = line_split[0].len();
			if depth > choices_stack.len() {
				choices_stack.push(ContainerEntry::Choice(vec![(choice, vec![])]));
			} else {
				choices_stack.last_mut().unwrap().add_entry((choice, vec![]));
			}
		} else {
			target_array.push(doc!{
				"type": "text",
				"text": line.trim(),
			});
		}
	}

	let story_doc = doc!{
		"data": story,
	};

	println!("{}", story_doc);

	let out_vec = story_doc.to_vec().unwrap();
	let mut outfile = File::create(filename.replace(".quilla", ".bson")).unwrap();
	outfile.write(&out_vec).unwrap();
}
