use std::io::Cursor;
use std::fs::read;

use bson::{Bson, Document};
use evalexpr::{Context, ContextWithMutableFunctions, ContextWithMutableVariables, DefaultNumericTypes, EvalexprError, HashMapContext};
use serde::{Deserialize, Serialize};

use crate::quilla_compiler::compile_story_to_struct;

enum AuxIndex {
	None,
	Choice(usize),
	If(usize),
}

struct NodeIndex {
	index: usize,
	aux_index: AuxIndex,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompiledStory {
	pub story: Vec<Bson>,
}

pub struct QuillaStory {
	story: Vec<Bson>,
	index_stack: Vec<NodeIndex>,

	selected_choice: usize,
	variables: HashMapContext,
}

impl QuillaStory {
	pub fn new(story: Vec<Bson>) -> Self {
		QuillaStory {
			story,
			index_stack: vec![NodeIndex{index: 0, aux_index: AuxIndex::None}],
			selected_choice: 0,
			variables: HashMapContext::new(),
		}
	}

	pub fn from_file(filename: &str) -> Self {
		let file_bytes = read(filename).unwrap();
		let cursor = Cursor::new(file_bytes);
		let story_data = bson::deserialize_from_reader::<_, CompiledStory>(cursor).unwrap();
		QuillaStory::new(story_data.story)
	}

	pub fn from_uncompiled_file(filename: &str) -> Result<Self, &str> {
		let compiled = compile_story_to_struct(filename)?;
		Ok(QuillaStory::new(compiled.story))
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
					"set" => {
						let name = current_node.get_str("name").unwrap();
						let value_str = current_node.get_str("value").unwrap();
						evalexpr::eval_empty_with_context_mut(&format!("{} = {}", name, value_str), &mut self.variables).unwrap();
					},
					"do" => {
						let what = current_node.get_str("what").unwrap();
						evalexpr::eval_empty_with_context(what, &self.variables).unwrap();
					},
					"if" => {
						let conditions = current_node.get_array("conditions").unwrap();
						let mut selected_index = usize::MAX;
						for i in 0..conditions.len() {
							let cond = conditions[i].as_str().unwrap();
							if evalexpr::eval_boolean_with_context(cond, &self.variables).unwrap() {
								selected_index = i;
								break;
							}
						}

						if selected_index != usize::MAX {
							self.index_stack.push(NodeIndex{ aux_index: AuxIndex::If(selected_index), index: 0 });
							continue;
						}
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

	pub fn get_variable(&self, name: &str) -> Option<&evalexpr::Value> {
		self.variables.get_value(name)
	}

	pub fn set_variable(&mut self, name: &str, value: evalexpr::Value) -> Result<(), EvalexprError> {
		self.variables.set_value(name.into(), value)
	}

	pub fn register_function(&mut self, name: &str, function: evalexpr::Function<DefaultNumericTypes>) -> Result<(), EvalexprError> {
		self.variables.set_function(name.into(), function)
	}

	////////////////////////////////////////////////////////////////////////////////////////////////
	
	fn get_current_node<'a>(&'a self, nodes: &'a Vec<Bson>, indices: &[NodeIndex]) -> Option<(&'a Document, usize)> {
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
				AuxIndex::If(index) => {
					nodes[indices[0].index].as_document().unwrap().get_array("branches").unwrap()[index].as_array().unwrap()
				},
			};

			self.get_current_node(new_nodes, &indices[1..indices.len()])
		}
	}
}
