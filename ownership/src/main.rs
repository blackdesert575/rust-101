// fn read(y: bool) {
//     if y {
//         println!("y is true!");
//     }
// }

// fn main() {
//     let x = true;
//     read(x);
// }

fn read(y: bool) {
    if y {
        println!("y is true!");
    }
}

fn main() {
    read(x); // oh no! x isn't defined!
    let x = true;
}