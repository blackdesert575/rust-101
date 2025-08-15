// Owenrship Rules
// Variable Scope
// fn main() {
//     let s = "hello";
// }
// // The String Type
// fn main() {
//     let s = String::from("hello");
// }

// string literals v.s the string type
fn main() {
    let mut s = String::from("hello");

    s.push_str(", world");

    println!("{s}");
}

// Memory and Allocation