

#[cfg(test)]
mod tests_runtime {
    use quilla::quilla_story::QuillaStory;

	#[test]
	fn test_basic_text() {
		let mut story = QuillaStory::from_uncompiled_file("tests/test_1.quilla").unwrap();
		assert_eq!(story.continue_story(), "Hello");
		assert_eq!(story.continue_story(), "this is a test");
		assert_eq!(story.continue_story(), "");
	}
}
