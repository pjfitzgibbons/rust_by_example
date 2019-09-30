#[allow(dead_code)]
fn main() {
  let pair = (4, 2);

  println!("Tell me about {:?}", pair);

  match pair {
    (x, y) if x == y => println!("These are twins", ),

    (x, y) if x + y == 0 => println!("Antimatter, kaboom!", ),

    (x, _) if x % 2 == 1 => println!("The first one is odd", ),

    _ => println!("No correlation...", ),
  }

  fn age() -> u32 {
    15
  }

  fn main() {
    println!("Tell me what tyep of person you are", );

    // match age() {
    //   0 => println!("I'm not born yet I guess", ),
    //   n @ 1 ..= 12  =>
    // }
  }
}