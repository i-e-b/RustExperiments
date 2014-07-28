
struct Point {
    x:f64,
    y:f64
}

fn main() {
    let mut mpoint = Point{x:1.0,y:1.0};
    let origin = Point{x:0.0, y:0.0};

    mpoint.y += 1.1;
    // origin.x = 1.0; // error- not mutable

    print_point(mpoint);
    print_point(origin);
}

fn print_point(p:Point) {
    match p {
        Point {x:0.0, y:vy} => println!("height {}", vy), // value match on x, name alias from y -> vy
        Point {x, y} => println!("{} {}", x, y) // use record names
    }
}

