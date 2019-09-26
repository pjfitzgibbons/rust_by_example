#![allow(dead_code)]

enum WebEvent {
  // unit-like members
  PageLoad,
  PageUnload,
  // tuple-like members
  KeyPress(char),
  Paste(String),
  // c-like structures
  Click { x: i64, y: i64 },
}

// takes a WebEvent enum and returns nothing
fn inspect(event: WebEvent) {
  match event {
    WebEvent::PageLoad => println!("page loaded",),
    WebEvent::PageUnload => println!("page unloaded",),
    // destructure `c` from inside the `enum`
    WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
    WebEvent::Paste(s) => println!("pasted \"{}\".", s),
    // destructure `click` into x and y
    WebEvent::Click { x, y } => println!("clicked at x={}, y={}.", x, y),
  }
}

enum Status {
  Rich,
  Poor,
}

enum Work {
  Civilian,
  Soldier,
}

enum Number {
  Zero,
  One,
  Two,
}

enum Color {
  Red = 0xff0000,
  Green = 0x00ff00,
  Blue = 0x0000ff,
}

fn main() {
  let pressed = WebEvent::KeyPress('x');
  // `to_owned()` creates an owned `String` from a string slice.
  let pasted = WebEvent::Paste("my text".to_owned());
  let click = WebEvent::Click { x: 20, y: 80 };
  let load = WebEvent::PageLoad;
  let unload = WebEvent::PageUnload;

  inspect(pressed);
  inspect(pasted);
  inspect(click);
  inspect(load);
  inspect(unload);

  use crate::Status::{Poor, Rich};
  use crate::Work::*;

  let status = Poor;
  let work = Civilian;

  match status {
    Rich => println!("The rich have lots of money!",),
    Poor => println!("The poor have no money..",),
  }

  match work {
    Civilian => println!("Civilians Work!",),
    Soldier => println!("Soldiers fight!",),
  }

  println!("zero is {}", Number::Zero as i32, );
  println!("one is {}", Number::One as i32 );
  println!("roses are #{:06x}", Color::Red as i32);
  println!("violets are #{:06x}", Color::Blue as i32);
  
}
