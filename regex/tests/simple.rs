mod util;

#[test]
fn test_plain() {
	let regex = util::setup("string");
	assert_regex_eq!(regex, "string");
	assert_regex_ne!(regex, "strong");
}

#[test]
fn test_match_any() {
	let regex = util::setup("str.ng");
	assert_regex_eq!(regex, "string");
	assert_regex_eq!(regex, "strong");
	assert_regex_ne!(regex, "spring");
}