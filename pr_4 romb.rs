const SIZE: usize = 5; 

fn main() {
  
    for i in 1..=SIZE {
      
        print!("{} ", " ".repeat(SIZE - i));

       
        println!("{}", "*".repeat(2 * i - 1));
    }


    for i in (1..SIZE).rev() {
        
        print!("{} ", " ".repeat(SIZE - i));

     
        println!("{}", "*".repeat(2 * i - 1));
    }
}
