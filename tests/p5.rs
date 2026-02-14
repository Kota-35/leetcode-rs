mod common;

use common::load_cases;
use leetcode_rs::solutions::p5::Solution;
use serde::Deserialize;

#[derive(Deserialize)]
struct Input {
    s: String,
}

#[test]
fn p5_longest_palindrome() {
    let cases = load_cases::<Input, String>("testcases/p5/cases.json");
    for case in cases.cases {
        let got = Solution::longest_palindrome(case.input.s);
        assert_eq!(got, case.output, "case {}", case.id);
    }
}
