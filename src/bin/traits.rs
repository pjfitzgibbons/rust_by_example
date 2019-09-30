use typename::TypeName;
use std::convert::From;
use std::fmt;

#[derive(Debug)]
struct Number {
  value: i32,
}
impl From<i32> for Number {
  fn from(item: i32) -> Self {
    Number { value: item }
  }
}

struct Circle {
  radius: i32
}

impl fmt::Display for Circle {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Circle of radius {}", self.radius)
  }
}

fn main() {
  let my_str = "hello";
  let my_string = String::from(my_str);

  println!("{:?}", my_str);
  println!("{:?}", my_string);
  println!("{}", my_str.type_name_of());
  println!("{}", my_string.type_name_of());

  let num = Number::from(30);
  println!("My number is {:?}", num);

  let int = 5;
  let num: Number = int.into();
  println!("My number is {:?}", num);

  let circle = Circle { radius: 6};
  println!("{}", circle.to_string() );

  let parsed: i32 = "5".parse().unwrap();
  let turbo_parsed = "10".parse::<i32>().unwrap();

  let sum = parsed + turbo_parsed;
  println!("Sum: {:?}", sum);  

  let x = 5u32;

  let y = {
      let x_squared = x * x;
      let x_cube = x_squared * x;

      // This expression will be assigned to `y`
      x_cube + x_squared + x
  };

  let z = {
      // The semicolon suppresses this expression and `()` is assigned to `z`
      2 * x;
  };

  println!("x is {:?}", x);
  println!("y is {:?}", y);
  println!("z is {:?}", z);  
}

