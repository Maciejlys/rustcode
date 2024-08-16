struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = numbers.len() - 1;

        while left < right {
            let res = numbers[right] + numbers[left];

            if res == target {
                return vec![left as i32 + 1, right as i32 + 1];
            } else if res > target {
                right -= 1;
            } else {
                left += 1;
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
        let numbers = vec![2, 7, 11, 15];
        let target = 9;
        let output = vec![1, 2];
        assert_eq!(Solution::two_sum(numbers, target), output);
    }

    #[test]
    fn test_case_2() {
        let numbers = vec![2, 3, 4];
        let target = 6;
        let output = vec![1, 3];
        assert_eq!(Solution::two_sum(numbers, target), output);
    }

    #[test]
    fn test_case_3() {
        let numbers = vec![-1, 0];
        let target = -1;
        let output = vec![1, 2];
        assert_eq!(Solution::two_sum(numbers, target), output);
    }
}
