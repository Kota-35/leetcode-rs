pub struct Solution;

impl Solution {
    // 問題:
    // Given a string s, return the longest palindromic substring in s.
    //
    // Example 1:
    // Input: s = "babad"
    // Output: "bab"
    // Explanation: "aba" is also a valid answer.
    //
    // Example 2:
    // Input: s = "cbbd"
    // Output: "bb"
    //
    // Constraints:
    // 1 <= s.length <= 1000
    // s consist of only digits and English letters.
    //
    // 方針(中心展開法):
    // 回文は「中心から左右に同じ文字が並ぶ」性質を持つ。
    // そのため、各位置を中心として左右に広げれば、その中心を持つ最長回文を求められる。
    //
    // ただし回文には2種類ある:
    // - 奇数長: 中心が1文字 (例: "aba")
    // - 偶数長: 中心が2文字の間 (例: "abba")
    // すべての i について
    // - (i, i) を中心に展開 (奇数)
    // - (i, i+1) を中心に展開 (偶数)
    // を試し、最長区間を更新する。
    //
    // アルゴリズム手順:
    // Step1: 文字列をバイト列として扱う (本問は英数字のみなのでバイト比較でOK)
    // Step2: expand(left, right) で左右に一致する限り拡張する
    // Step3: 各中心で得られた区間長を比較し、最長 [best_start, best_end) を保持する
    // Step4: 最後に最長区間を切り出して返す
    //
    // 計算量:
    // - 時間計算量: O(n^2)
    //   (各中心で最悪 O(n) 展開、中心は O(n) 個)
    // - 空間計算量: O(1)
    pub fn longest_palindrome(s: String) -> String {
        let bytes = s.as_bytes();
        let n = bytes.len();
        if n < 2 {
            return s;
        }

        // 指定した中心 (left, right) から外側へ広げ、
        // 回文である最大の半開区間 [start, end) を返す。
        fn expand(
            bytes: &[u8],
            mut left: i32,
            mut right: i32,
        ) -> (usize, usize) {
            let n = bytes.len() as i32;
            while left >= 0
                && right < n
                && bytes[left as usize] == bytes[right as usize]
            {
                left -= 1;
                right += 1;
            }
            ((left + 1) as usize, right as usize)
        }

        let mut best_start = 0usize;
        let mut best_end = 1usize; // 半開区間 [start, end)

        for i in 0..n {
            // 奇数長回文: 中心が文字 i
            let (l1, r1) = expand(bytes, i as i32, i as i32);
            if r1 - l1 > best_end - best_start {
                best_start = l1;
                best_end = r1;
            }

            if i + 1 < n {
                // 偶数長回文: 中心が i と i+1 の間
                let (l2, r2) = expand(bytes, i as i32, i as i32 + 1);
                if r2 - l2 > best_end - best_start {
                    best_start = l2;
                    best_end = r2;
                }
            }
        }

        s[best_start..best_end].to_string()
    }
}
