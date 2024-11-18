fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false; 
    }
    if n == 2 || n == 3 {
        return true; // прості числа
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    
    let limit = (n as f64).sqrt() as u32;
    for i in (5..=limit).step_by(6) {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_numbers() {
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(7), true);
        assert_eq!(is_prime(11), true);
        assert_eq!(is_prime(13), true);
    }

    #[test]
    fn test_non_prime_numbers() {
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(6), false);
        assert_eq!(is_prime(8), false);
        assert_eq!(is_prime(9), false);
        assert_eq!(is_prime(15), false);
    }
}
