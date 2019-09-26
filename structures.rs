
#[derive(Debug)]
struct Person<'a> {
  name: &'a str,
  age: u8,
}

// A unit struct
#[derive(Debug)]
#[allow(dead_code)]
struct Nil;

#[derive(Debug)]
#[allow(dead_code)]
struct Pair(i32, i32);

#[derive(Copy, Clone, Debug)]
#[allow(dead_code)]
struct Point {
  x: f32,
  y: f32,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
  p1: Point,
  p2: Point,
}

fn main() {
  // struct with field-init shorthand
  let name = "Peter";
  let age = 27;
  let peter = Person { name, age };

  println!("{:?}", peter);

  // initiate a Point
  let point: Point = Point { x: 0.3, y: 0.4 };
  println!("point coordinates: ({}, {})", point.x, point.y);

  // struct update syntax
  let new_point: Point = Point { x: 1.1, ..point };
  println!("New point coordinates: ({}, {})", new_point.x, new_point.y);

  // Destructure the point using a `let` binding
  let Point { x: my_x, y: my_y } = point;

  let _rectangle = Rectangle {
    // struct instantiation is also an expression
    p1: Point { x: my_y, y: my_x },
    p2: point,
  };

  println!("{:?}", point);
  println!("my_x: {:?}, my_y: {:?}", my_x, my_y)

}