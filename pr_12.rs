use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng(); 
    (0..n).map(|_| rng.gen_range(10..100)).collect() 
}



fn min_adjacent_sum(data: &[i32]) -> i32 {
    let mut min_sum = i32::MAX; // Початково задаємо найбільше можливе значення для мінімуму
    for window in data.windows(2) { // windows(2) дає пари сусідніх елементів
        let sum = window[0] + window[1];
        min_sum = min_sum.min(sum); // Оновлюємо мінімум
    }
    min_sum
}

fn print_data(data: &[i32]) {
    println!("Data: {:?}", data);
    let min_sum = min_adjacent_sum(data);
    println!("Minimum adjacent sum: {}", min_sum);
}

fn main() {
    let n = 20;
    let random_vector = gen_random_vector(n);
    
    print_data(&random_vector);
}
