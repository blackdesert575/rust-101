// hello, world
// if you don't have a main function, it will show this error:
// error[E0601]: `main` function not found in crate `comments`

fn main() {
    println!("Hello, world!");
    let lucky_number = 7; // I’m feeling lucky today
    println!("lucky_number:{}", lucky_number);
    println!("lucky_number:{lucky_number}");

/*     Rust also has another kind of comment, 
    documentation comments, 
    which we’ll discuss in the “Publishing a Crate to Crates.io” section of Chapter 14. */
    // https://rust-book.cs.brown.edu/ch14-02-publishing-to-crates-io.html
}