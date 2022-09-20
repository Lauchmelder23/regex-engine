#![feature(test)]

extern crate test;

mod tests {
	use super::*;
	use test::Bencher;
	use regex::regex::Regex;

	fn setup_matcher(pattern: &str) -> Regex {
		let regex = Regex::new(pattern);
		assert!(regex.is_ok());
		regex.unwrap()
	}

	#[bench]
	fn bench_match_simple(b: &mut Bencher) {
		let regex = setup_matcher("Paris");

		b.iter(|| regex.test("Paris"));
	}

	#[bench]
	fn bench_match_quantifiers(b: &mut Bencher) {
		let regex = setup_matcher("P(ar+(is)*)*");

		b.iter(|| regex.test("Parrrrisisisisaris"));
	}

	#[bench]
	fn bench_match_disjunctions(b: &mut Bencher) {
		let regex = setup_matcher("Paris|London|Berlin|Warsaw");

		b.iter(|| regex.test("Berlin"));
	}

	#[bench]
	fn bench_match_complex(b: &mut Bencher) {
		let regex = setup_matcher("P(ar+(is)*)*|L(on+(do)*)*n|B(e+((rl)*i)*n)*|W*((ar)+(s)*aw*)*");

		b.iter(|| regex.test("Beeeirliinen"));
	}
}