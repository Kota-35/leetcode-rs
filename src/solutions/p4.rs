use std::cmp::{max, min};

pub struct Solution;

impl Solution {
    // 問題:
    // 昇順にソートされた配列 nums1, nums2 が与えられる。
    // 2 つを合わせた全体の中央値を返す。
    // 本来の要求計算量は O(log(m+n))。
    //
    // 例:
    // 入力: nums1 = [1,3], nums2 = [2]
    // 出力: 2.00000
    // 説明: マージ後は [1,2,3] で、中央値は 2。
    //
    // 入力: nums1 = [1,2], nums2 = [3,4]
    // 出力: 2.50000
    // 説明: マージ後は [1,2,3,4] で、中央値は (2 + 3) / 2 = 2.5。
    //
    // 方針:
    // 2配列の「分割位置」を二分探索する。
    //
    // 1. nums1 を短い方、nums2 を長い方にする
    // 2. nums1 の分割位置 i を二分探索で決める
    // 3. nums2 側は j = (m+n+1)/2 - i で連動する
    // 4. 条件
    //    - left1 <= right2
    //    - left2 <= right1
    //    が満たされれば正しい分割
    // 5. 正しい分割が見つかったら中央値を計算
    //    - 全体長が奇数: max(left1, left2)
    //    - 偶数: (max(left1,left2) + min(right1,right2)) / 2
    //
    // 計算量:
    // 短い方の配列長を m とすると O(log m) = O(log(m+n))
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // a を短い方に固定すると、探索範囲が最小になり境界処理もしやすい
        let (a, b) = if nums1.len() <= nums2.len() {
            (nums1, nums2)
        } else {
            (nums2, nums1)
        };

        let (m, n) = (a.len(), b.len());
        let half = (m + n + 1) / 2;

        // 二分探索で i (a 側の切れ目)を探索する
        let mut lo: usize = 0;
        let mut hi = m;

        while lo <= hi {
            let i = (lo + hi) / 2;
            let j = half - i;

            let left1 = if i == 0 { i32::MIN } else { a[i - 1] };
            let right1 = if i == m { i32::MAX } else { a[i] };
            let left2 = if j == 0 { i32::MIN } else { b[j - 1] };
            let right2 = if j == n { i32::MAX } else { b[j] };

            if left1 <= right2 && left2 <= right1 {
                if (m + n) % 2 == 1 {
                    return max(left1, left2) as f64;
                }
                return (max(left1, left2) as f64
                    + min(right1, right2) as f64)
                    / 2.0;
            } else if left1 > right2 {
                hi = i.saturating_sub(1);
            } else {
                lo = i + 1;
            }
        }

        unreachable!("inputs must be sorted and at least one array non-empty");
    }
}
