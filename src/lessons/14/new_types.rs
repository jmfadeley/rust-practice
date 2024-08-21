/// Lessons "14.7 NEw Type Idiom"
/// https://doc.rust-lang.org/rust-by-example/generics/new_types.html

// The newtype idiom gives compile time guarantees that the right type of value is supplied to a program.
// For example, an age verification function that checks age in years, must be given a value of type Years.

struct Years(i64);

struct Days(i64);

impl Years {
  pub fn to_days(&self) -> Days {
    Days(self.0 * 365)
  }
}

impl Days {
  // truncates partial years
  pub fn to_years(&self) -> Years {
    Years(self.0 / 365)
  }
}

fn is_adult(age: &Years) -> bool {
  age.0 >= 18
}

fn main() {
  let age = Years(25);
  let age_days = age.to_days();
  println!("Is an adult? {}", is_adult(&age));
  println!("Is an adult? {}", is_adult(&age_days.to_years()));
  // println!("Is an adult? {}", is_adult(&age_days));
  // ^ Uncomment for good time. Not.
}

// To obtain the newtypes' value as a base type, you may use the tuple or destructured syntax like so:
// fn main() {
//   let years = Years(42);
//   let years_as_primitive_1: i64 = years.0; //Tuple
//   let Years(years_as_primitive_2) = years;
// }