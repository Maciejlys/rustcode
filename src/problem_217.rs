use std::collections::HashSet;
struct Solution {}

#[allow(dead_code)]
impl Solution {
    fn contains_duplicate(nums: Vec<i32>) -> bool {
        return nums.len() != HashSet::<i32>::from_iter(nums).len();
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_1() {
        let input = vec![1, 2, 3, 1];
        let output = true;
        assert_eq!(Solution::contains_duplicate(input), output);
    }

    #[test]
    fn test_case_2() {
        let input = vec![1, 2, 3, 4];
        let output = false;
        assert_eq!(Solution::contains_duplicate(input), output);
    }

    #[test]
    fn test_case_3() {
        let input = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        let output = true;
        assert_eq!(Solution::contains_duplicate(input), output);
    }
}
