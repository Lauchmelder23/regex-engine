#![feature(test)]

extern crate test;

mod tests {
	use super::*;
	use test::Bencher;
	use regex::regex::Regex;

	#[bench]
	fn bench_parse_simple(b: &mut Bencher) {
		b.iter(|| Regex::new("Paris"));
	}

	#[bench]
	fn bench_parse_quantifiers(b: &mut Bencher) {
		b.iter(|| Regex::new("P(ar+(is)*)*"));
	}

	#[bench]
	fn bench_parse_disjunctions(b: &mut Bencher) {
		b.iter(|| Regex::new("Paris|London|Berlin|Warsaw"));
	}

	#[bench]
	fn bench_parse_complex(b: &mut Bencher) {
		b.iter(|| Regex::new("P(ar+(is)*)*|L(on+(do)*)*n|B(e+((rl)*i)*n)*|W*((ar)+(s)*aw*)*"));
	}

}