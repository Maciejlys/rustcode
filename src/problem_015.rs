struct Solution;

use std::collections::HashSet;

#[allow(dead_code)]
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = HashSet::<Vec<i32>>::new();
        nums.sort();

        for i in 0..nums.len() {
            let num = nums[i];
            if num > 0 {
                break;
            }

            if i > 0 && num == nums[i - 1] {
                continue;
            }

            let mut left = i + 1;
            let mut right = nums.len() - 1;

            while left < right {
                let sum = num + nums[left] + nums[right];

                if sum == 0 {
                    res.insert(vec![num, nums[left], nums[right]]);
                    left += 1;
                    right -= 1;

                    while nums[left] == nums[left - 1] && left < right {
                        left += 1;
                    }
                }

                if sum < 0 {
                    left += 1;
                }

                if sum > 0 {
                    right -= 1;
                }
            }
        }

        Vec::from_iter(res)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_2() {
        let input = vec![0, 1, 1];
        let output: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::three_sum(input), output);
    }

    #[test]
    fn test_case_3() {
        let input = vec![0, 0, 0];
        let output: Vec<Vec<i32>> = vec![input.clone()];
        assert_eq!(Solution::three_sum(input), output);
    }
}
