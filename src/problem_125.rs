struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let parsed: String = s
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect();
        parsed.to_owned() == parsed.chars().rev().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_1() {
        let input = "A man, a plan, a canal: Panama".to_string();
        let output = true;
        assert_eq!(Solution::is_palindrome(input), output);
    }

    #[test]
    fn test_case_2() {
        let input = "race a car".to_string();
        let output = false;
        assert_eq!(Solution::is_palindrome(input), output);
    }

    #[test]
    fn test_case_3() {
        let input = " ".to_string();
        let output = true;
        assert_eq!(Solution::is_palindrome(input), output);
    }
}
