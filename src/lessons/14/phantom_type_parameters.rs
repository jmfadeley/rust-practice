/// Lessons "14.9 Phantom Type Parameters"
/// https://doc.rust-lang.org/rust-by-example/generics/phantom.html

/*
A phatnonm type parameter is one that doesn't show up at runtime, but is checked statically (and only) at compile time.

Data types can use extra generic type parameters to act as markers of to perform type checking at compile time. These extra
parameters hold no storage values, and have no runtime behavior.

In the following example, we combine `std::marker::PhantomData` with the phantom type parameter concept tp create tuples 
containing different data types.*/

use std::marker::PhantomData;

// A phantom tuple structure which is generic over A with hidden parameter B.
#[derive(PartialEq)]
struct PhantomTuple<A, B>(A, PhantomData<B>);

// A phantom struct which is generic over A with hidden Parameter B.

#[derive(PartialEq)]
struct PhantomStruct<A, B> { first: A, phantom: PhantomData<B>}

// Note: Storage is allocated for generic type A but not for B, therefore B cannot be used in computations.

fn main() {
  // Here f32 and f64 are the hidden parameters
  // PhantomTuple type specified as <char, f32>
  // then char and f64
  let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
  let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

  let _struct1: PhantomStruct<char, f32> = PhantomStruct {
    first: 'Q',
    phantom: PhantomData,
  };

  let _struct2: PhantomStruct<char, f64> = PhantomStruct {
    first: 'Q', 
    phantom: PhantomData,
  };

  // compile tile error: Type mismatch so these cannot be compared:
  // println!("_tuple1 == _tuple2 yields: {}", _tuple1 == _tuple2);
  // println!("_struct1 == _struct2 yields: {}", _struct1 == _struct2);
}