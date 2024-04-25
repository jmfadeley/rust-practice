/// Lessons "4.4 Freezing"
/// Sources: https://doc.rust-lang.org/rust-by-example/variable_bindings/freeze.html

// When data is bound by the same name immutably, it also freezes. Frozne data can't be 
// modifie duntil the immutable binding goes out of scope.

fn main() {
    let mut _mutable_integer = 7i32;

    {
        // Shadowing by immutable `_mutable_integer`:
        let _mutable_integer = _mutable_integer;
        // _mutable_integer = 50;
        // This will cause errors.
        // And now we go out of scope...
    }
    // Now it's not frozen and you can again change the value.
    _mutable_integer = 3;
}
