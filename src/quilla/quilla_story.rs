use std::{collections::HashMap};

use bson::{Bson, Document};

use crate::variant::Variant;

enum AuxIndex {
	None,
	Choice(usize),
	If(bool),
}

struct NodeIndex {
	index: usize,
	aux_index: AuxIndex,
}

pub struct QuillaStory<'a> {
	story: &'a Vec<Bson>,
	index_stack: Vec<NodeIndex>,

	selected_choice: usize,
	variables: HashMap<String, Variant>,
}

impl<'a> QuillaStory<'a> {
	pub fn new(story_doc: &'a Document) -> Self {
		QuillaStory {
			story: story_doc.get_array("data").unwrap(),
			index_stack: vec![NodeIndex{index: 0, aux_index: AuxIndex::None}],
			selected_choice: 0,
			variables: HashMap::new(),
		}
	}

	pub fn can_continue(&self) -> bool {
		if let Some((current_node, _)) = self.get_current_node(&self.story, &self.index_stack) {
			current_node.get_str("type").unwrap() != "choice"
		} else {
			false
		}
	}

	pub fn continue_story(&mut self) -> String {
		loop {
			if let Some((current_node, _)) = self.get_current_node(&self.story, &self.index_stack) {
				match current_node.get_str("type").unwrap() {
					"text" => {
						let result = current_node.get_str("text").unwrap().into();
						
						loop {
							let new_array_size = self.get_current_node(&self.story, &self.index_stack).unwrap().1;
							self.index_stack.last_mut().unwrap().index += 1;
							if self.index_stack.last_mut().unwrap().index >= new_array_size {
								println!("TEST");
								self.index_stack.pop().unwrap();
								if self.index_stack.is_empty() {
									break;
								}
							} else {
								break;
							}
						}

						return result;
					},
					"choice" => {
						return String::new();
					},
					"var" => {
						let name = current_node.get_str("name").unwrap();
						let value_str = current_node.get_str("value").unwrap();
						let value = Variant::parse(value_str).unwrap();
						self.set_variable(name.into(), value);
					},
					"set" => {
						let name = current_node.get_str("name").unwrap();
						let value_str = current_node.get_str("value").unwrap();
						let value = Variant::parse(value_str).unwrap();
						self.set_variable(name.into(), value);
					},
					_ => {

					},
				}

				self.index_stack.last_mut().unwrap().index += 1;
			} else {
				return String::new();
			}
		}
	}

	pub fn continue_story_maximally(&mut self) -> Vec<String> {
		let mut result = vec![];
		while self.can_continue() {
			result.push(self.continue_story());
		}

		result
	}

	pub fn get_current_choices(&self) -> Vec<String> {
		if let Some((current_node, _)) = self.get_current_node(&self.story, &self.index_stack) {
			let empty = vec![];
			let choices = current_node.get_array("choices").unwrap_or(&empty);
			choices.iter().map(|ch| ch.as_str().unwrap().into()).collect()
		} else {
			vec![]
		}
	}

	pub fn select_choice(&mut self, index: usize) {
		if let Some((current_node, _)) = self.get_current_node(&self.story, &self.index_stack) {
			if current_node.get_str("type").unwrap() == "choice" {
				self.selected_choice = index;
				self.index_stack.push(NodeIndex{ aux_index: AuxIndex::Choice(index), index: 0 });
			}
		}
	}

	pub fn get_variable(&self, name: &String) -> Option<&Variant> {
		self.variables.get(name)
	}

	pub fn set_variable(&mut self, name: String, value: Variant) {
		self.variables.insert(name, value);
	}

	////////////////////////////////////////////////////////////////////////////////////////////////
	
	fn get_current_node(&self, nodes: &'a Vec<Bson>, indices: &[NodeIndex]) -> Option<(&Document, usize)> {
		if indices.is_empty() || (*indices.last().unwrap()).index >= nodes.len() {
			None
		} else if indices.len() == 1 {
			Some((nodes[indices[0].index].as_document().unwrap(), nodes.len()))
		} else {
			let new_nodes = match indices[1].aux_index {
				AuxIndex::None => {
					panic!();
				},
				AuxIndex::Choice(index) => {
					nodes[indices[0].index].as_document().unwrap().get_array("results").unwrap()[index].as_array().unwrap()
				},
				AuxIndex::If(passed) => {
					nodes[indices[0].index].as_document().unwrap().get_array(if passed { "if" } else { "else" }).unwrap()
				},
			};

			self.get_current_node(new_nodes, &indices[1..indices.len()])
		}
	}
}
