/*
67. Add Binary
Easy
Given two binary strings a and b, return their sum as a binary string.



Example 1:

Input: a = "11", b = "1"
Output: "100"
Example 2:

Input: a = "1010", b = "1011"
Output: "10101"


Constraints:

1 <= a.length, b.length <= 104
a and b consist only of '0' or '1' characters.
Each string does not contain leading zeros except for the zero itself.
 */
struct Solution {}
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        if a.is_empty() {
            return b;
        }
        if b.is_empty() {
            return a;
        }
        let mut result = "".to_string();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_test_cases() {
        let result = Solution::add_binary("".to_string(), "1".to_string());
        assert_eq!(result, "1".to_string());
        let result = Solution::add_binary("11".to_string(), "".to_string());
        assert_eq!(result, "11".to_string());
    }

    #[test]
    fn default_test_cases() {
        let result = Solution::add_binary("11".to_string(), "1".to_string());
        assert_eq!(result, "100".to_string());
        let result = Solution::add_binary("1010".to_string(), "1011".to_string());
        assert_eq!(result, "10101".to_string());
    }
}