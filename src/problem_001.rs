use std::collections::HashMap;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            match map.get(num) {
                Some(&j) => return vec![j as i32, i as i32],
                None => {
                    map.insert(target - num, i as i32);
                }
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let output = vec![0, 1];
        assert_eq!(Solution::two_sum(nums, target), output);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let output = vec![1, 2];
        assert_eq!(Solution::two_sum(nums, target), output);
    }

    #[test]
    fn test_case_3() {
        let nums = vec![3, 3];
        let target = 6;
        let output = vec![0, 1];
        assert_eq!(Solution::two_sum(nums, target), output);
    }
}
