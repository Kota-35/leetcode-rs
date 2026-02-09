mod common;

use common::load_cases;
use leetcode_rs::solutions::p1::Solution;
use serde::Deserialize;

#[derive(Deserialize)]
struct Input {
    nums: Vec<i32>,
    target: i32,
}

#[test]
fn p1_two_sum() {
    let cases = load_cases::<Input, Vec<i32>>("testcases/p1/cases.json");
    for case in cases.cases {
        let got = Solution::two_sum(case.input.nums, case.input.target);
        assert_eq!(got, case.output, "case {}", case.id);
    }
}
