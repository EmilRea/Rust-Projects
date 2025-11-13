#[derive(Debug)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}

fn calculate_total_area(shapes: &[Shape]) -> f64 {


    let mut total_area: f64 = 0.0;
    for shape in shapes {
        match shape {
            Shape::Circle { radius } => total_area += radius * radius * std::f64::consts::PI,
            Shape::Rectangle { width, height }=> total_area += width * height,
        }
    }
    total_area
}

fn main() {
    let shapes = vec![
        Shape::Rectangle { width: 10.0, height: 5.0 },
        Shape::Circle { radius: 3.0 },
    ];

    let total_area = calculate_total_area(&shapes);
    println!("The total area of all shapes is: {}", total_area);
}