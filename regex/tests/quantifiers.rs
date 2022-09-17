mod util;

#[test]
fn test_optional() {
	let regex = util::setup("stri?ng");
	assert_regex_eq!(regex, "string");
	assert_regex_eq!(regex, "strng");
	assert_regex_ne!(regex, "striiing");
}

#[test]
fn test_one_or_more() {
	let regex = util::setup("Bana+na");
	assert_regex_eq!(regex, "Banana");
	assert_regex_eq!(regex, "Banaaaana");
	assert_regex_ne!(regex, "Banna");

	let regex_with_groups = util::setup("Ba(na)+na");
	assert_regex_eq!(regex_with_groups, "Banana");
	assert_regex_eq!(regex_with_groups, "Bananananana");
	assert_regex_ne!(regex_with_groups, "Banaaaana");
	assert_regex_ne!(regex_with_groups, "Banna");

	let nested_quantifiers = util::setup("Ba(na+)+na");
	assert_regex_eq!(nested_quantifiers, "Banaaanaaaana");
	assert_regex_eq!(nested_quantifiers, "Banaaaaanaanaaanaaana");
	assert_regex_ne!(nested_quantifiers, "Banna");
	assert_regex_ne!(nested_quantifiers, "Banaananaaa");
}

#[test]
fn test_zero_or_more() {
	let regex = util::setup("Bana*na");
	assert_regex_eq!(regex, "Banana");
	assert_regex_eq!(regex, "Banaaaana");
	assert_regex_eq!(regex, "Banna");

	let regex_with_groups = util::setup("Ba(na)*na");
	assert_regex_eq!(regex_with_groups, "Banana");
	assert_regex_eq!(regex_with_groups, "Bananananana");
	assert_regex_eq!(regex_with_groups, "Bana");
	assert_regex_ne!(regex_with_groups, "Banaaaana");
	assert_regex_ne!(regex_with_groups, "Banna");

	let nested_quantifiers = util::setup("Ba(na*)*na");
	assert_regex_eq!(nested_quantifiers, "Banaaanaaaana");
	assert_regex_eq!(nested_quantifiers, "Banaaaaanaanaaanaaana");
	assert_regex_eq!(nested_quantifiers, "Bannnnnnnnnna");
	assert_regex_ne!(nested_quantifiers, "Banbna");
	assert_regex_ne!(nested_quantifiers, "Banaananaaa");
}

#[test]
fn test_multiple() {
	let regex = util::setup("P(ar+(is)*)*");

	assert_regex_eq!(regex, "Paris");
	assert_regex_eq!(regex, "Parrrris");
	assert_regex_eq!(regex, "Parrrr");
	assert_regex_eq!(regex, "Parrisisis");
	assert_regex_eq!(regex, "Parrrrrrisisisarrrrrrisisis");
	assert_regex_eq!(regex, "Parisarisisarisisis");
	assert_regex_eq!(regex, "Parrisarr");

	assert_regex_ne!(regex, "Pais");
	assert_regex_ne!(regex, "Parisisa");
	assert_regex_ne!(regex, "Parsi");
}