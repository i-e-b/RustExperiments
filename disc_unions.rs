#[deriving(Debug)]
enum Direction {
    Up, Down, Left, Right
}

struct Point {
    x:f64, y:f64
}

enum Shape {
    Circle {centre:Point, radius:f64},
    Arrow {centre:Point, direction:Direction}
}

fn write_shape(s:Shape) {
    match s {
        Shape::Circle {centre, radius} => println!("Circle {},{}:{}",centre.x,centre.y,radius),
        _ => println!("not a circle")
    }
}

fn main() {
    // println!("{:?}", Direction::Down);  // not sure how to fix this: the trait `core::fmt::Show` is not implemented for the type `Direction`
    write_shape(Shape::Circle {centre:Point{x:1.0,y:1.0}, radius:1.0});
}
