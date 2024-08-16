use std::collections::HashMap;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut result: HashMap<String, Vec<String>> = HashMap::new();

        for s in strs {
            let mut vec: Vec<char> = s.chars().collect();
            vec.sort();
            let sorted = vec.into_iter().collect();
            result.entry(sorted).or_insert_with(Vec::new).push(s);
        }

        result.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_2() {
        let input = vec![""].into_iter().map(|x| x.to_string()).collect();
        let output = vec![vec![""]];
        assert_eq!(Solution::group_anagrams(input), output);
    }

    #[test]
    fn test_case_3() {
        let input = vec!["a"].into_iter().map(|x| x.to_string()).collect();
        let output = vec![vec!["a"]];
        assert_eq!(Solution::group_anagrams(input), output);
    }
}
