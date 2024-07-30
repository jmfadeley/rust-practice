// These tests are running concurrently, which means the results can be wrong. This is by design to illustrate 
// thread safety considerations while testing.

#[cfg(test)]
mod tests {
  use std::fs::OpenOptions;
  use std::io::Write;

  #[test]
  fn test_file() {
    let mut file = OpenOptions::new()
      .append(true)
      .create(true)
      .open("ferris.txt")
      .expect("Failed to open ferris.txt");

    // Print "Ferris" five times.
    for _ in 0..5 {
      file.write_all("Ferris\n".as_bytes())
        .expect("Could not write to ferris.txt");
    }
  }

  #[test]
  fn test_file_also() {
    // Opens the same file or creates it if it doesn't exist.
    let mut file = OpenOptions::new()
      .append(true)
      .create(true)
      .open("ferris.txt")
      .expect("Failed to open ferris.txt");

    for _ in 0..5 {
      file.write_all("Corro\n".as_bytes())
        .expect("Could not write to ferris.txt");
    }
  }

  #[test]
  fn west() {
    print!("Nothing.");
  }
}