// Safety is the Absence of Undefined Behavior

// fn read(y: bool) {
//     if y {
//         println!("y is true!");
//     }
// }

// fn main() {
//     let x = true;
//     read(x);
// }

// fn read(y: bool) {
//     if y {
//         println!("y is true!");
//     }
// }

// fn main() {
//     read(x); // oh no! x isn't defined!
//     let x = true;    
// }

// Variables Live in the Stack
// fn main() {
//     let n = 5;
//     let y = plus_one(n);
//     println!("The value of y is: {y}");
// }

// fn plus_one(x: i32) -> i32 {
//     x + 1
// }

// fn main() {
// let a = 5;
// let mut b = a;
// b += 1;
// }

// Boxes Live in the Heap
// fn main() {
// let a = [0; 1_000_000];
// let b = a;
// }

// fn main() {
// let a = Box::new([0; 1_000_000]);
// let b = a;
// }

// Rust Does Not Permit Manual Memory Management
// fn free<T>(_t: T) {}
// fn main() {
// let b = Box::new([0; 100]);
// free(b);
// assert!(b[0] == 0);
// }

// A Box’s Owner Manages Deallocation
// fn main() {
//     let a_num = 4;
//     make_and_drop();
// }

// fn make_and_drop() {
//     let a_box = Box::new(5);
// }

// Collections Use Boxes
// fn main() {
//     let first = String::from("Ferris");
//     let full = add_suffix(first);
//     println!("{full}");
// }

// fn add_suffix(mut name: String) -> String {
//     name.push_str(" Jr.");
//     name
// }

// Variables Cannot Be Used After Being Moved
// fn main() {
//     let first = String::from("Ferris");
//     let full = add_suffix(first);
//     println!("{full}, originally {first}"); // first is now used here
// }

// fn add_suffix(mut name: String) -> String {
//     name.push_str(" Jr.");
//     name
// }

// Moved heap data principle: if a variable x moves ownership of heap data to another variable y, then x cannot be used after the move.

// Cloning Avoids Moves
// fn main() {
//     let first = String::from("Ferris");
//     let first_clone = first.clone();
//     let full = add_suffix(first_clone);
//     println!("{full}, originally {first}");
// }

// fn add_suffix(mut name: String) -> String {
//     name.push_str(" Jr.");
//     name
// }

// Question 2

// fn add_suffix(mut s: String) -> String {
//   s.push_str(" world");
//   s
// }
// fn main() {
//   let s = String::from("hello");
//   let s2 = add_suffix(s);
//   println!("{}", s2);
// }

// Question 3
// fn main() {
//   let s = String::from("hello");
//   let s2;
//   let b = false;
//   if b {
//     s2 = s;
//   }
//   println!("{}", s);
// }

// Question 4


fn move_a_box(b: Box<i32>) {
  // This space intentionally left blank              
}

fn main() {
    // let b = Box::new(0);
    // move_a_box(b);
    // println!("{}", b);

    // let b = Box::new(0);
    // let b2 = b;
    // move_a_box(b);

    let b = Box::new(0);
    let b2 = b;
    println!("{}", b);
    move_a_box(b2);

    // let b = Box::new(0);
    // move_a_box(b);
    // let b2 = b;
}