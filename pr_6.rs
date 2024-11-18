
fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let num1 = 56;
    let num2 = 98;

    println!("The GCD of {} and {} is {}", num1, num2, gcd(num1, num2));
}
