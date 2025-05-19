#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let optional_point = Some(Point { x: 100, y: 200 });

    match optional_point {
        Some(ref p) => println!("Co-ordinates are {},{}", p.x, p.y),
        _ => panic!("No match!"),
    }

    // without the ref ownership of the point changes to p in the
    // match statement, thus, we cannot use p here
    println!("{optional_point:?}"); // Don't change this line.
}
