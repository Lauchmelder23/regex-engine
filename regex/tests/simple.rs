mod util;

#[test]
fn test_plain() {
	let regex = util::setup("string");
	assert_match!(regex, "string", 6);
	assert_failure!(regex, "strong");
}

#[test]
fn test_match_any() {
	let regex = util::setup("str.ng");
	assert_match!(regex, "string", 6);
	assert_match!(regex, "strong", 6);
	assert_failure!(regex, "spring");
}