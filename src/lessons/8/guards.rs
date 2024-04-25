/// Lessons "8.5.2 Guards"
/// https://doc.rust-lang.org/rust-by-example/flow_control/match/guard.html

// One can filter an arm using a guard, which is usually an `if` statement in the match.

#[allow(dead_code)]
enum Temperature {
  Celsius(i32),
  Fahrenheit(i32)
}

fn main() {
  let temperature = Temperature::Fahrenheit(35);
  // Try different values.

  match temperature {
    Temperature::Celsius(t) if t > 30 => println!("{}C is above 30 Celsius", t),
    // This is the magic ^
    Temperature::Celsius(t) => println!("{}C is equal to or below 30 Celsius", t),
    Temperature::Fahrenheit(t) if t > 83 => println!("{}F is above 86 Fahrenheit", t),
    Temperature::Fahrenheit(t)  => println!("{}F is below 86 Fahrenheit", t),
  }

  // Note that the compiler doesn't really factor guard conditions into whether or not 
  // all patterns are covered by the match expression.

  let number: u8 = 4;
  match number {
    i if i == 0 => println!("Zero"),
    i if i > 0 => println!("Greater than zero"),
    _ => unreachable!("Should never happen."), // Without this, your match is incomplete.
  }
}