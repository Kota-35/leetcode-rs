mod common;

use common::load_cases;
use leetcode_rs::solutions::p4::Solution;
use serde::Deserialize;

#[derive(Deserialize)]
struct Input {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
}

#[test]
fn p4_find_median_sorted_arrays() {
    let cases = load_cases::<Input, f64>("testcases/p4/cases.json");
    for case in cases.cases {
        let got = Solution::find_median_sorted_arrays(
            case.input.nums1,
            case.input.nums2,
        );
        assert_eq!(got, case.output, "case {}", case.id);
    }
}
