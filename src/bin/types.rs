#![allow(overflowing_literals)]

fn main() {
  let decimal = 65.4321_f32;

  // let integer: u8 = decimal;
  // FIXME ^

  let integer = decimal as u8;
  let character = integer as char;

  println!("Casting: {} -> {} -> {}", decimal, integer, character);

  // when casting to any unsigned type T,
  // std::T::MAX + 1 is added or subtracted until the value
  // fits into the new type

  // 1000 already fits in a u16
  println!("1000 as a u16 is: {}", 1000 as u16);

  // 1000 - 256 - 256 - 256 = 232
  // under the hood, the first 8 least-significant bits (LS8) are kept
  // the rest towards most-significant bit (MS8) are truncated
  println!("1000 as a u8 is: {}", 1000 as u8);
  println!("1000 % 256 is: {}", 1000 % 256);
  // -1 + 256 = 255
  println!("  -1 as u8 is: {}", (-1i8) as u8);

  // when casting to a signed type, the (bitwise) result is the same
  // as first casting to the corresponding unsigned type.  If the MSB 
  // is 1 then the value is negative

  // unless it already fits
  println!("128 as an i16 is: {}", 128 as i16);
  // 128 as u8 -> 128, who's two's complement in 8 bits is:
  println!("128 as an i8 is: {}", 128 as i8);

  println!("1000 as u8 is: {}", 1000 as u8);
  println!("232 as i8 is: {}", 232 as i8);

  // Suffixed literals, their types are known at initialization
  let x = 1u8;
  let y = 2u32;
  let z = 3f32;

  // Unsuffixed literal, their types depend on how they are used
  let i = 1;
  let f = 1.0;

  // `size_of_val` returns the size of a variable in bytes
  println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
  println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
  println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
  println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
  println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));

  // Because of the annotation, the compiler knows that `elem` has type u8.
  let elem = 5u8;

  // Create an empty vector (a growable array).
  let mut vec = Vec::new();
  // At this point the compiler doesn't know the exact type of `vec`, it
  // just knows that it's a vector of something (`Vec<_>`).

  // Insert `elem` in the vector.
  vec.push(elem);
  // Aha! Now the compiler knows that `vec` is a vector of `u8`s (`Vec<u8>`)
  // TODO ^ Try commenting out the `vec.push(elem)` line

  println!("{:?}", vec);
}