#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut sum;
        let mut carry = 0;
        // ダミーヘッド。ノードの追加処理を簡単にするため。
        let mut l3 = Some(Box::new(ListNode::new(0)));
        let mut head = l3.as_mut();

        let (mut l1, mut l2) = (l1.as_ref(), l2.as_ref());

        // どちらかに桁が残っている間、両方を走査する。
        while l1.is_some() || l2.is_some() {
            sum = 0;

            if let Some(node) = l1 {
                sum += node.val;
                // l1 を次へ進める。
                l1 = node.next.as_ref()
            }

            if let Some(node) = l2 {
                sum += node.val;
                // l2 を次へ進める。
                l2 = node.next.as_ref()
            }

            sum += carry;
            carry = sum / 10;
            sum %= 10;
            // 現在の桁を追加する。
            head.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum)));
            head = head.unwrap().next.as_mut()
        }

        // 繰り上がりが残っていれば追加する。
        if carry != 0 {
            head.as_mut().unwrap().next = Some(Box::new(ListNode::new(carry)))
        }

        // ダミーヘッドを捨てて実際の先頭を返す。
        l3.unwrap().next
    }
}
