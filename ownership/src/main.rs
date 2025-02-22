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

fn main() {
let a = Box::new([0; 1_000_000]);
let b = a;
}