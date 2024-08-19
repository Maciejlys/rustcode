use crate::util::linked_list::ListNode;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (Some(n), None) | (None, Some(n)) => Some(n),
            (Some(l1), Some(l2)) => {
                if l1.val >= l2.val {
                    Some(Box::new(ListNode {
                        val: l2.val,
                        next: Solution::merge_two_lists(Some(l1), l2.next),
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: l1.val,
                        next: Solution::merge_two_lists(l1.next, Some(l2)),
                    }))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{linked, util::linked_list::to_list};

    use super::Solution;

    #[test]
    fn test_case_1() {
        assert_eq!(
            Solution::merge_two_lists(linked![1, 2, 4], linked![1, 3, 4]),
            linked![1, 1, 2, 3, 4, 4]
        );
    }
}
