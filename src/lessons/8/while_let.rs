/// Lessons "8.8 While Let"
/// Sources: https://doc.rust-lang.org/rust-by-example/flow_control/while_let.html

// Like if let, while let can make awkward match sequences more tolerable. 
// Like if we were dealing with sequences that increment i:

fn main() {
  let mut optional = Some(0);

  // This sucks.
  loop {
    match optional {
      Some(i) => {
        if i > 9 {
          println!("Great than 9, quit.");
          optional = None;
        } else {
          println!("`i` is `{:?}`, try again.", i);
          optional = Some(i + 1);
        } // ^ This approach takes 3 indentations.
      },
      _ => { break; } // Required, sucks.
    }
  }

  // Or, use while let.
  optional = Some(0);

  while let Some(i) = optional {
    if i > 9 {
      println!("Greater than 9, quit!");
      optional = None;
    } else {
      println!("`i` is `{:?}`, try again.", i);
      optional = Some(i + 1);
    }
  }
}