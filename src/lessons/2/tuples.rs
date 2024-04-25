fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
        -1i8, -2i16, -3i32, -4i64);
    // Values can use tuple indexing.
    // Note that you can output regular arguments as long as they're fmt::display
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);

    // Look at the power of what you CAN do!
    // And if you EVER do this, then so help me...
    let tuple_of_tuples = ((1u8, 2u16, 3u32), (4u64, -1i8), -2i16);

    // Look ma! Printable! With debug only, need to impl fmt::Display otherwise.
    println!("Pair is {:?}", tuple_of_tuples);

    // Limit of 12 elements though on the println. More than that, errors.
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("Too long tuple: {:?}", too_long_tuple);

    let pair = (1, true);
    println!("Pair is {:?}", pair);

    println!("The reversed pair is {:?}", reverse(pair));

    // If you need a tuple with one element, end it with a comma. Otherwise,
    // the compiler will think you're just adding a literal with parantheses.
    println!("One element tuple: {:?}", (5u32,));
    println!("Just a lone, sad integer: {:?}", (5u32));

    // You can also destructure tuples to create bindings.
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    // Because these are primitives under the hood, you don't have to use debug.
    // But it is probably safer. 
    println!("{}, {}, {}, {}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
}