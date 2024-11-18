
struct Point {
    x: i32,
    y: i32,
}

struct Rectangle {
    a: Point, 
    b: Point, 
}

impl Rectangle {
    
    fn area(&self) -> i32 {
        (self.b.x - self.a.x).abs() * (self.b.y - self.a.y).abs()
    }

    fn overlap(&self, other: &Rectangle) -> Option<i32> {
        let x_overlap = std::cmp::max(0, std::cmp::min(self.b.x, other.b.x) - std::cmp::max(self.a.x, other.a.x));
        let y_overlap = std::cmp::max(0, std::cmp::min(self.b.y, other.b.y) - std::cmp::max(self.a.y, other.a.y));

        if x_overlap > 0 && y_overlap > 0 {
            Some(x_overlap * y_overlap)
        } else {
            None 
        }
    }
}

fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut total_area = 0;

    for rect in xs.iter() {
        total_area += rect.area();
    }

    let mut overlap_area = 0;

    for i in 0..xs.len() {
        for j in i+1..xs.len() {
            if let Some(overlap) = xs[i].overlap(&xs[j]) {
                overlap_area += overlap;
            }
        }
    }

    total_area - overlap_area
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
}

fn main() {
    area_occupied_test();
    println!("Test passed!");
}
