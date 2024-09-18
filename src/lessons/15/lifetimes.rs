/// Lessons "15.4 Lifetimes"
/// https://doc.rust-lang.org/rust-by-example/scope/lifetime.html

/* A lifetime is a construct the compiler (or rather its borrow checker) uses to ensure all
borrows are valid. Specifically, a variables' lifetime begins when it is created and ends when
it is destroyed. While lifetimes and scopes are often referred together, tehy are not the same.

Take for example the case where we borrow a variable via &. The borrow has a lifetime that is
determines by where it is declared. As a result, teh borrow is valdi as long as it ends before
the lender is destroyed. However the scope of the borrow is determiend by where the reference 
is used.

In the following example, and in the rest of this section, we will see how lifetimes relate to 
scopes, as well as how the two differ. */

// Lifetimes are annotated below with liens denoting the creation and destruction of each variable.
// i has the longest lifetime because its scope entirely encloses both borrow1 and borrow2. The 
// duration of borrow1 compared to borrow2 is irrelevant since they are disjoint.

fn main() {
  let i = 3; // Lifetime for i starts.

  {
    let borrow1 = &i; // Borrow lifetime starts.
    println!("borrow1: {}", borrow1);
  } // Borrow1 lifetime ends.

  {
    let borrow2 = &i;
    println!("borrow2: {}", borrow2);
  } // Borrow2 lifetime ends.
}

// Note that no names or types are assigned to label lifetimes. This restricts how lifetimes
// will be able to be used as we will see.