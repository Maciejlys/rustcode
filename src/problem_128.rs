struct Solution;
use std::collections::HashSet;

#[allow(dead_code)]
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = HashSet::from_iter(nums.to_owned());
        let mut max = 0;

        for num in nums {
            let to_find = num + 1;
            if set.contains(&to_find) {
                continue;
            }

            let mut counter = 1;
            let curr = num.to_owned();
            let mut find = curr - 1;

            while set.contains(&find) {
                counter += 1;
                find -= 1;
            }

            max = std::cmp::max(max, counter);
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_1() {
        let input = vec![100, 4, 200, 1, 3, 2];
        let output = 4;
        assert_eq!(Solution::longest_consecutive(input), output);
    }

    #[test]
    fn test_case_2() {
        let input = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
        let output = 9;
        assert_eq!(Solution::longest_consecutive(input), output);
    }
}
