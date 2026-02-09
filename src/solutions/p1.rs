pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut seen = HashMap::new();
        for (i, n) in nums.iter().enumerate() {
            let need = target - n;
            if let Some(&j) = seen.get(&need) {
                return vec![j as i32, i as i32];
            }
            seen.insert(*n, i);
        }
        vec![]
    }
}
