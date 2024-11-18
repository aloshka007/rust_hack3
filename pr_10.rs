fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false; 
    }
    if n == 2 || n == 3 {
        return true; 
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false; 
    }
    
    // Перевірка від 5 до √n
    let limit = (n as f64).sqrt() as u32;
    for i in (5..=limit).step_by(6) { 
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
    }

    true
}
