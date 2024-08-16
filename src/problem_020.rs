struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for i in s.chars() {
            match i {
                '[' => stack.push(']'),
                '(' => stack.push(')'),
                '{' => stack.push('}'),
                '}' | ')' | ']' if Some(i) != stack.pop() => return false,
                _ => {}
            }
        }
        return stack.is_empty();
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_1() {
        let input = "()".to_string();
        let output = true;
        assert_eq!(Solution::is_valid(input), output);
    }

    #[test]
    fn test_case_2() {
        let input = "()[]{}".to_string();
        let output = true;
        assert_eq!(Solution::is_valid(input), output);
    }

    #[test]
    fn test_case_3() {
        let input = "(]".to_string();
        let output = false;
        assert_eq!(Solution::is_valid(input), output);
    }
}
