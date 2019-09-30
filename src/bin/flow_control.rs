#![allow(unreachable_code)]

fn main() {
  let n = 50;

  if n < 0 {
    print!("{} is negative", n);
  } else if n > 0 {
    print!("{} is positive", n);
  } else {
    print!("{} is zero", n);
  }

  let big_n = if n < 10 && n > -10 {
    println!(", and is a small number, increase ten-fold");

    10 * n
  } else {
    println!(", and is  a big number, halve the number");
    n / 2
  };

  println!("{} -> {}", n, big_n);

  let mut count = 0u32;

  println!("Let's count until infinity!");

  loop {
    count += 1;

    if count == 3 {
      println!("three",);
      continue;
    }

    println!("{}", count);

    if count == 5 {
      println!("OK, that's enough",);

      break;
    }
  }

  'outer: loop {
    println!("Entered the outer loop", );

    'inner: loop {
      println!("Entered inner loop", );

      break 'outer;
    }
    println!("This point will never be reached");
  }

  println!("Exited the outer loop", );


  let mut counter = 0;

  let result = loop {
    counter += 1;

    if counter == 10 {
      break counter * 2;
    }
  };

  assert_eq!(result, 20);

  // let mut n = 1;

  for n in 1..=100 {
    if n % 15 == 0 {
      println!("fizzbuzz", );
    } else if n % 3 == 0 {
      println!("fizz", );
    } else if n % 5 == 0 {
      println!("buzz", );
    } else {
      println!("{}", n);
    }
  }

  let mut names = vec!["Bob", "Frank", "Ferris"];

  for name in names.iter_mut() {
    *name = match name {
      &mut "Ferris" => "There is a rustacean among us!",
      _ => "Hello",
    }
  }

  println!("names {:?}", names);
}
