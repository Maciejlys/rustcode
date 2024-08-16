struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut map = std::collections::HashMap::new();
        s.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);
        t.chars().for_each(|c| *map.entry(c).or_insert(0) -= 1);
        map.into_values().all(|v| v == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_1() {
        let s = "car".to_string();
        let t = "rat".to_string();
        let output = false;
        assert_eq!(Solution::is_anagram(s, t), output);
    }

    #[test]
    fn test_case_2() {
        let s = "anagram".to_string();
        let t = "nagaram".to_string();
        let output = true;
        assert_eq!(Solution::is_anagram(s, t), output);
    }
}
