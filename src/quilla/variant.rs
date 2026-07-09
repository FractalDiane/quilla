use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Variant {
	Bool(bool),
	Int(i64),
	Float(f64),
	Str(String),
}

impl Variant {
	pub fn parse(string: &str) -> Option<Self> {
		if string == "true" {
			Some(Variant::Bool(true))
		} else if string == "false" {
			Some(Variant::Bool(false))
		} else if let Ok(int) = string.parse::<i64>() {
			Some(Variant::Int(int))
		} else if let Ok(float) = string.parse::<f64>() {
			Some(Variant::Float(float))
		} else if string.starts_with('"') && string.ends_with('"') {
			Some(Variant::Str(string[1..string.len() - 1].into()))
		} else {
			None
		}
	}
}
