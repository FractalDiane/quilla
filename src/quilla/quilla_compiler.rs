use std::io::{BufRead, BufReader, Write};
use std::fs::File;

use bson::{Bson, doc};

use crate::quilla_story::CompiledStory;

enum ContainerEntry {
	Choice(Vec<(String, Vec<Bson>)>),
	If(Vec<(String, Vec<Bson>)>, Vec<Bson>),
}

impl ContainerEntry {
	pub fn add_entry(&mut self, entry: (String, Vec<Bson>)) {
		match self {
			ContainerEntry::Choice(choices) => {
				choices.push(entry);
			},
			ContainerEntry::If(branches, _) => {
				branches.push(entry);
			},
		}
	}

	pub fn get_item_array(&mut self) -> &mut Vec<Bson> {
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

pub fn compile_story_to_struct(path: &str) -> Result<CompiledStory, &str> {
	let file = BufReader::new(File::open(&path).unwrap());

	let mut story = Vec::<Bson>::new();
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
								"results": choices.iter().map(|ch| ch.1.clone()).collect::<Vec<Vec<Bson>>>(),
							}.into());
						},
						ContainerEntry::If(branches, _) => {
							target_array.push(doc!{
								"type": "if",
								"conditions": branches.iter().map(|ch| ch.0.clone()).collect::<Vec<String>>(),
								"branches": branches.iter().map(|ch| ch.1.clone()).collect::<Vec<Vec<Bson>>>(),
							}.into());
						},
					}
				}
			}
		}

		let target_array = if choices_stack.is_empty() {
			&mut story
		} else {
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
					}.into());
				},
				"SET" => {
					target_array.push(doc!{
						"type": "set",
						"name": line_split[1],
						"value": line_split[3],
					}.into());
				},
				"IF" => {
					target_array.push(doc!{
						"type": "if",
						"condition": line_split[1..].join(" "),
					}.into());
				},
				_ => {

				},
			}
		} else if line_split[0].starts_with('+') {
			let choice = line_split[1..].join(" ");
			if indent_level + 1 > choices_stack.len() {
				choices_stack.push(ContainerEntry::Choice(vec![(choice, vec![])]));
			} else {
				choices_stack.last_mut().unwrap().add_entry((choice, vec![]));
			}
		} else {
			target_array.push(doc!{
				"type": "text",
				"text": line.trim(),
			}.into());
		}
	}

	let compiled_story = CompiledStory{story};
	Ok(compiled_story)
}

pub fn compile_story_to_file(path: &str) -> Result<(), &str> {
	let compiled_story = compile_story_to_struct(path)?;
	let out_vec = bson::serialize_to_vec(&compiled_story).unwrap();
	let mut outfile = File::create(path.replace(".quilla", ".bson")).unwrap();
	outfile.write(&out_vec).unwrap();

	Ok(())
}
