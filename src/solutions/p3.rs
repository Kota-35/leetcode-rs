use std::cmp::max;

pub struct Solution;

impl Solution {
    // 問題:
    // 文字列 s が与えられる。重複する文字を含まない部分文字列のうち、
    // 最長の長さを返す。
    //
    // 例:
    // 入力: s = "abcabcbb"
    // 出力: 3
    // 説明: 最長は "abc"（長さ 3）。
    //       "bca" や "cab" も条件を満たす。
    //
    // 制約:
    // 0 <= s.length <= 5 * 10^4
    // s は英字・数字・記号・空白からなる。
    //
    // Rust特有のポイント:
    // - String は UTF-8 なので、添字で直接 1 文字アクセスできない。
    // - そのため s.as_bytes() でバイト列化し、bytes[j] で O(1) 参照する。
    // - vis を [bool; 128] にしているのは ASCII 想定のため。
    //   （問題文の文字種を ASCII とみなして処理している）
    // - Unicode 全般を厳密に扱うなら、char 単位 + HashSet などの実装が必要。
    //
    // 方針:
    // 開始位置 i を固定し、右端 j を伸ばして重複が出るまで探索する。
    // vis 配列で「その区間に文字が既に出たか」を管理する。
    pub fn length_of_longest_substring(s: String) -> i32 {
        let bytes = s.as_bytes();
        let n = bytes.len();
        // res: これまでに見つかった「重複なし部分文字列」の最長長さ
        let mut res: usize = 0;

        for i in 0..n {
            // ASCII コード(0..=127)の出現フラグ
            let mut vis = [false; 128]; // ASCII　想定

            for j in i..n {
                let idx = bytes[j] as usize;
                if vis[idx] {
                    break;
                }
                vis[idx] = true;
                // 現在の区間 [i, j] の長さで最長値を更新
                res = max(res, j - i + 1);
            }
        }

        res as i32
    }
}
