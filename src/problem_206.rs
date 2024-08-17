use crate::util::linked_list::ListNode;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;

        while let Some(mut h) = head {
            head = h.next.take();
            h.next = prev;
            prev = Some(h);
        }

        prev
    }
}

#[cfg(test)]
mod tests {
    use crate::{linked, util::linked_list::to_list};

    use super::Solution;

    #[test]
    fn test_case_1() {
        assert_eq!(
            Solution::reverse_list(linked![1, 2, 3, 4, 5]),
            linked![5, 4, 3, 2, 1]
        );
    }
}
