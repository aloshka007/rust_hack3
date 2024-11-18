fn is_palindrome(n: u32) -> bool {
    let s = n.to_string();          
    let rev_s: String = s.chars().rev().collect(); // Перевертаємо рядок
    s == rev_s                       
}
