
fn main() {
  // assign a reference of type i32
  let reference = &4;

  match reference {
    // if reference is matched against `&val`, it results
    // in a comparison like:
    // `&i32` or `&val`
    // ^ we see that if the matching `&`'s are dropped, then the `i32`
    // should be assigned to `val`.
    &val => println!("Got a value via destructuring: {:?}", val),
  }
  // To avoid teh `&`, dereference before matching.
  match *reference {
    val => println!("Got a value via destructuring: {:?}", val), 
  }

  // state of variable as a ref depends on RHS if there are no other
  // markers on LHS
  let _not_a_reference = 3;

  // `ref` keyword will create the reference
  let ref _is_a_reference = 3;

  // values without references can be referenced by `ref` and `ref mut`
  let value = 5;
  let mut mut_value = 6;

  // use `ref` keyword to create a reference
  match value {
    ref r => println!("Got a reference to a value {:?}", r),
  }

  // use `ref mut` similarily
  match mut_value {
    ref mut m => {
      // Got a reference.  Must dereference before modifying it
      *m += 10;
      println!("We added 10. `mut_value`: {:?}", m);
    }
  }
}