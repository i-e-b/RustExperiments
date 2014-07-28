#![feature(struct_variant)]

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
        Circle {centre, radius} => println!("Circle {},{}:{}",centre.x,centre.y,radius),
        _ => println!("not a circle")
    }
}

fn main() {
    println!("{:?}", Down);
    write_shape(Circle {centre:Point{x:1.0,y:1.0}, radius:1.0});
}
