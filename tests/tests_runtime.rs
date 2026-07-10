macro_rules! s {
	($path:expr) => {
		QuillaStory::from_uncompiled_file($path).unwrap()
	};
}

macro_rules! assert_text {
	($st:expr, $text:expr) => {
		assert_eq!($st.continue_story(), $text)
	};
}

#[cfg(test)]
mod tests_runtime {
    use quilla::quilla_story::QuillaStory;

	#[test]
	fn test_basic_text() {
		let mut story = s!("tests/test_1_text.quilla");
		assert_text!(story, "Hello");
		assert_text!(story, "this is a test");
	}

	#[test]
	fn test_single_choice() {
		let mut story = s!("tests/test_2_single_choice.quilla");
		assert_text!(story, "Hello");
	}
}
