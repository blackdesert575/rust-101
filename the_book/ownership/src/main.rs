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


// fn move_a_box(b: Box<i32>) {
//   // This space intentionally left blank              
// }

// fn main() {
//     // let b = Box::new(0);
//     // move_a_box(b);
//     // println!("{}", b);

//     // let b = Box::new(0);
//     // let b2 = b;
//     // move_a_box(b);

//     let b = Box::new(0);
//     let b2 = b;
//     println!("{}", b);
//     move_a_box(b2);

//     // let b = Box::new(0);
//     // move_a_box(b);
//     // let b2 = b;
// }

// Rust data structures
// Box
// Vec
// String
// HashMap

// References and Borrowing

// fn main() {
//     let m1 = String::from("Hello");
//     let m2 = String::from("world");
//     greet(m1, m2);
//     let s = format!("{} {}", m1, m2); // Error: m1 and m2 are moved
// }

// fn greet(g1: String, g2: String) {
//     println!("{} {}!", g1, g2);
// }

// fn main() {
//     let m1 = String::from("Hello");
//     let m2 = String::from("world");
//     let (m1_again, m2_again) = greet(m1, m2);
//     let s = format!("{} {}", m1_again, m2_again);
// }

// fn greet(g1: String, g2: String) -> (String, String) {
//     println!("{} {}!", g1, g2);
//     (g1, g2)
// }

// References Are Non-Owning Pointers

// fn main() {
//     let m1 = String::from("Hello");
//     let m2 = String::from("world");
//     greet(&m1, &m2); // note the ampersands
//     let s = format!("{} {}", m1, m2);
// }

// fn greet(g1: &String, g2: &String) { // note the ampersands
//     println!("{} {}!", g1, g2);
// }

// Dereferencing a Pointer Accesses Its Data

// fn main() {
// let mut x: Box<i32> = Box::new(1);
// let a: i32 = *x;         // *x reads the heap value, so a = 1
// *x += 1;                 // *x on the left-side modifies the heap value,
//                          //     so x points to the value 2

// let r1: &Box<i32> = &x;  // r1 points to x on the stack
// let b: i32 = **r1;       // two dereferences get us to the heap value

// let r2: &i32 = &*x;      // r2 points to the heap value directly
// let c: i32 = *r2;    // so only one dereference is needed to read it
// }

// fn main()  {
// let x: Box<i32> = Box::new(-1);
// let x_abs1 = i32::abs(*x); // explicit dereference
// let x_abs2 = x.abs();      // implicit dereference
// assert_eq!(x_abs1, x_abs2);

// let r: &Box<i32> = &x;
// let r_abs1 = i32::abs(**r); // explicit dereference (twice)
// let r_abs2 = r.abs();       // implicit dereference (twice)
// assert_eq!(r_abs1, r_abs2);

// let s = String::from("Hello");
// let s_len1 = str::len(&s); // explicit reference
// let s_len2 = s.len();      // implicit reference
// assert_eq!(s_len1, s_len2);
// }


// Rust Avoids Simultaneous Aliasing and Mutation
// fn main() {
// let mut v: Vec<i32> = vec![1, 2, 3];
// v.push(4);
// }

fn main() {
    // let mut s = String::from("hello");
    let  s = String::from("hello");

    let r1 = &s; // 不可變借用
    let r2 = &s; // 不可變借用
    // let r3 = &mut s; // ❌ 錯誤：嘗試可變借用

    // println!("{}, {}, and {}", r1, r2, r3);
    println!("{}, {}", r1, r2);
}


// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s;
//     let r2 = &s;
//     println!("{} and {}", r1, r2); // ✅ 這裡 r1 和 r2 的作用範圍結束

//     let r3 = &mut s; // ✅ 這裡才取得可變借用
//     println!("{}", r3);
// }

// fn main() {
//     let mut s = String::from("hello");

//     let r = &mut s; // ✅ 只有可變借用
//     println!("{}", r);
// }
