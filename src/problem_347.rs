use std::{collections::HashMap, usize};

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut frequency: HashMap<i32, i32> = HashMap::new();

        nums.into_iter()
            .for_each(|n| *frequency.entry(n).or_insert(0) += 1);

        let mut counts: Vec<(i32, i32)> = frequency.into_iter().collect();
        counts.sort_by(|a, b| b.1.cmp(&a.1));

        let result: Vec<i32> = counts.into_iter().map(|(num, _)| num).collect();

        result.into_iter().take(k as usize).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_2() {
        let nums = vec![1];
        let k = 1;
        let output = vec![1];
        assert_eq!(Solution::top_k_frequent(nums, k), output);
    }
}
