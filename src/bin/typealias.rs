type Nanosecond = u64;
type Inch = u64;

type U64T = u64;

fn main() {
  let nanoseconds: Nanosecond = 5 as U64T;
  let inches: Inch = 2 as U64T;

  println!(
    "{} nanoseconds + {} inches = {} unit?",
    nanoseconds,
    inches,
    nanoseconds + inches
  );
}
