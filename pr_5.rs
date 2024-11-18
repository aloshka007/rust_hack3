const WIDTH: usize = 14; // Ширина конверта (необхідно для правильного відображення)
const HEIGHT: usize = 7; // Висота конверта

fn main() {
    
    for i in 0..HEIGHT {
        
        print!("{}", "*");

       
        print!("{}", " ".repeat(2 * i - 1));

       
        if i != 0 {
            print!("{}", "*");
        }

        
        println!();
    }

    
    for _ in 0..WIDTH {
        println!("{}", "*".repeat(WIDTH * 2 - 1)); // Виводимо широку лінію
    }
}
