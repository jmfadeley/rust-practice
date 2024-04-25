/// Lessons "8.5.1.5 Destructuring Structures"
/// https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring/destructure_structures.html

fn main() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    let fool = Foo { x: (1, 2), y: 3};

    match fool {
        Foo { x: (1, b), y} => println!("First of x is 1, b = {}, y = {}", b, y),

        // You can destructure structs and rename the variables, order doesn't matter
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
        // But you must tell the match how to deal with x in some form. This will cause error 
        // because no mention of x
        // Foo { y } => println!("EVERYONE MUST BE HERE! y = {}", y),
    }

    let faa = Foo { x: (1, 2), y: 3};

    // You don't need a match block to destructure structs:
    let Foo { x: x0, y: y0} = faa;
    println!("Outside: x0 = {x0:?}, y0 = {y0:?}");

    // Destructuring works with nested structs as well:
    struct Bar {
        foo: Foo,
    }

    let bar = Bar { foo: faa };
    let Bar { foo: Foo { x: nested_x, y: nested_y }} = bar;

    println!("Nested: nested_x = {nested_x:?}, nested_y = {nested_y:?}");

}