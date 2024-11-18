#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindromes() {
        assert_eq!(is_palindrome(121), true);  // 121 - паліндром
        assert_eq!(is_palindrome(12321), true); // 12321 - паліндром
        assert_eq!(is_palindrome(5), true);    // 5 - паліндром (одне число)
    }

    #[test]
    fn test_non_palindromes() {
        assert_eq!(is_palindrome(123), false);  // 123 - не паліндром
        assert_eq!(is_palindrome(98765), false); // 98765 - не паліндром
        assert_eq!(is_palindrome(10), false);   // 10 - не паліндром
    }
}
