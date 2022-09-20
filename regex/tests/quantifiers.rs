mod util;

/*
// #[test]
fn test_optional() {
	let regex = util::setup("stri?ng");
	assert_match!(regex, "string");
	assert_match!(regex, "strng");
	assert_failure!(regex, "striiing");
}
*/

#[test]
fn test_one_or_more() {
	let regex = util::setup("Bana+na");
	assert_match!(regex, "Banana", 6);
	assert_match!(regex, "Banaaaana", 9);
	assert_failure!(regex, "Banna");

	let regex_with_groups = util::setup("Ba(na)+na");
	assert_match!(regex_with_groups, "Banana", 6);
	assert_match!(regex_with_groups, "Bananananana", 12);
	assert_failure!(regex_with_groups, "Banaaaana");
	assert_failure!(regex_with_groups, "Banna");

	let nested_quantifiers = util::setup("Ba(na+)+na");
	assert_match!(nested_quantifiers, "Banaaanaaaana", 13);
	assert_match!(nested_quantifiers, "Banaaaaanaanaaanaaana", 21);
	assert_match!(nested_quantifiers, "Banaananaaa", 9);
	assert_failure!(nested_quantifiers, "Banna");
}

#[test]
fn test_zero_or_more() {
	let regex = util::setup("Bana*na");
	assert_match!(regex, "Banana", 6);
	assert_match!(regex, "Banaaaana", 9);
	assert_match!(regex, "Banna", 5);

	let regex_with_groups = util::setup("Ba(na)*na");
	assert_match!(regex_with_groups, "Banana", 6);
	assert_match!(regex_with_groups, "Bananananana", 12);
	assert_match!(regex_with_groups, "Bana", 4);
	assert_match!(regex_with_groups, "Banaaaana", 4);
	assert_failure!(regex_with_groups, "Banna");

	let nested_quantifiers = util::setup("Ba(na*)*na");
	assert_match!(nested_quantifiers, "Banaaanaaaana", 13);
	assert_match!(nested_quantifiers, "Banaaaaanaanaaanaaana", 21);
	assert_match!(nested_quantifiers, "Bannnnnnnnnna", 13);
	assert_match!(nested_quantifiers, "Banaananaaa", 9);
	assert_failure!(nested_quantifiers, "Banbna");
}


#[test]
fn test_multiple() {
	let regex = util::setup("P(ar+(is)*)*");

	assert_match!(regex, "Paris", 5);
	assert_match!(regex, "Parrrris", 8);
	assert_match!(regex, "Parrrr", 6);
	assert_match!(regex, "Parrisisis", 10);
	assert_match!(regex, "Parrrrrrisisisarrrrrrisisis", 27);
	assert_match!(regex, "Parisarisisarisisis", 19);
	assert_match!(regex, "Parrisarr", 9);

	assert_match!(regex, "Pais", 1);
	assert_match!(regex, "Parisisa", 7);
	assert_match!(regex, "Parsi", 3);
}

#[test]
fn test_wildcard() {
	let regex = util::setup("P.+ris");

	assert_match!(regex, "Paris", 5);
	assert_match!(regex, "Paoris", 6);
	assert_match!(regex, "Pawawaris", 9);

	let nested = util::setup("P(a.+)+ris");

	assert_match!(nested, "Parris", 6);
	assert_match!(nested, "Paorairis", 9);
	assert_match!(nested, "Pawawaris", 9);
	assert_match!(nested, "Paoaris", 7);

	assert_failure!(nested, "Paris");
}
