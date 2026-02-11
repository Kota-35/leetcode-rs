mod common;

use common::load_cases;
use leetcode_rs::solutions::p3::Solution;
use serde::Deserialize;

#[derive(Deserialize)]
struct Input {
    s: String,
}

#[test]
fn p3_length_of_longest_substring() {
    let cases = load_cases::<Input, i32>("testcases/p3/cases.json");
    for case in cases.cases {
        let got = Solution::length_of_longest_substring(case.input.s);
        assert_eq!(got, case.output, "case {}", case.id);
    }
}
