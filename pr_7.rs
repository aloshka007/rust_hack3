fn draw_tree(triangles: usize) {
    
    let triangle_height = 5; 
    let mut stars = 1;

  
    for _ in 0..triangles {
        for i in 0..triangle_height {
            
            let spaces = (triangle_height - i - 1) as usize;

            print!("{}", " ".repeat(spaces));

        
            println!("{}", "*".repeat(stars));
          
            stars += 2;
        }

         stars = 1;
    }
}

fn main() {
    let triangles = 5; // Кількість трикутників в ялинці
    draw_tree(triangles);
}
