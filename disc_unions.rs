use std::fmt;

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
    write_shape(Shape::Circle {centre:Point{x:1.0,y:1.0}, radius:1.0});
    println!("The only way is {}!", Direction::Down);
}

// Add a display trait to the 'Direction' enum
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self { // `&` gives us a borrowed reference. `*` dereferences to a value
            Direction::Up => write!(f, "up"),
            Direction::Down => write!(f, "down"),
            Direction::Left => write!(f, "left"),
            Direction::Right => write!(f, "right")
        }
    }
}

