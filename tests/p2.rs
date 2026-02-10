mod common;

use common::load_cases;
use leetcode_rs::solutions::p2::{ListNode, Solution};
use serde::Deserialize;

#[derive(Deserialize)]
struct Input {
    l1: Vec<i32>,
    l2: Vec<i32>,
}

fn vec_to_list(v: &[i32]) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    for &x in v.iter().rev() {
        head = Some(Box::new(ListNode { val: x, next: head }));
    }
    head
}

fn list_to_vec(mut node: Option<Box<ListNode>>) -> Vec<i32> {
    let mut out = Vec::new();
    while let Some(n) = node {
        out.push(n.val);
        node = n.next;
    }
    out
}

#[test]
fn p2_add_two_numbers() {
    let cases = load_cases::<Input, Vec<i32>>("testcases/p2/cases.json");
    for case in cases.cases {
        let l1 = vec_to_list(&case.input.l1);
        let l2 = vec_to_list(&case.input.l2);
        let got = Solution::add_two_numbers(l1, l2);
        assert_eq!(list_to_vec(got), case.output, "case {}", case.id);
    }
}
