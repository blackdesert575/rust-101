use std::any::type_name;
use std::io;

fn print_type<T>(_: &T) {
    println!("Type: {}", type_name::<T>());
}

fn main() {
    //Scalar Types
    //Integer Types
    // let guess = "42".parse().expect("Not a number!");
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess:{guess}");

    // 8-bit
    let i8_val: i8 = -128;
    let u8_val: u8 = 255;

    // 16-bit
    let i16_val: i16 = -32_768;
    let u16_val: u16 = 65_535;

    // 32-bit
    let i32_val: i32 = -2_147_483_648;
    let u32_val: u32 = 4_294_967_295;

    // 64-bit
    let i64_val: i64 = -9_223_372_036_854_775_808;
    let u64_val: u64 = 18_446_744_073_709_551_615;

    // 128-bit
    let i128_val: i128 = -170_141_183_460_469_231_731_687_303_715_884_105_728;
    let u128_val: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;

    // Architecture-dependent (isize & usize)
    let isize_val: isize = -9223372036854775808; // On a 64-bit system
    let usize_val: usize = 18_446_744_073_709_551_615; // On a 64-bit system

    // 印出變數及其類型
    println!("8-bit:    i8 = {} , u8 = {}", i8_val, u8_val);
    println!("16-bit:  i16 = {} , u16 = {}", i16_val, u16_val);
    println!("32-bit:  i32 = {} , u32 = {}", i32_val, u32_val);
    println!("64-bit:  i64 = {} , u64 = {}", i64_val, u64_val);
    println!("128-bit: i128 = {} , u128 = {}", i128_val, u128_val);
    println!("Arch:    isize = {} , usize = {}", isize_val, usize_val);

    //  Integer Literals in Rust
    // 各種數字字面量
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A'; // u8 型別

    // 印出每個數字字面量及其值
    println!("Decimal  (10進制)  : {}", decimal);
    println!("Hex      (16進制)  : {}", hex);
    println!("Octal    (8進制)   : {}", octal);
    println!("Binary   (2進制)   : {}", binary);
    println!("Byte     (u8)      : {} (ASCII: '{}')", byte, byte as char);    


    // Floating-Point Types
    // IEEE-754 standard
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    println!("x:{x}");
    println!("y:{y}");
    // print_type(&x);
    // print_type(&y);
    
    // Type check with print_type
    let num = 42;
    let float_num = 3.14;
    let text = "Hello, Rust!";
    
    print_type(&num);
    print_type(&float_num);
    print_type(&text);

    //Numeric Operations
    // addition
    let sum = 5 + 10;
    println!("5 + 10 = {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("95.5 - 4.3 = {difference}");

    // multiplication
    let product = 4 * 30;
    println!("4 * 30 = {product}");

    // division
    let quotient = 56.7 / 32.2;
    println!("56.7 / 32.2 = {quotient}");
    let truncated = -5 / 3; // Results in -1
    println!("-5 / 3 = {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("43 % 5 = {remainder}");

    //The Boolean Type
    let t = true;
    println!("t:{t}");
    let f: bool = false; // with explicit type annotation
    println!("f:{f}");
    //The Character Type
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("c:{c}, z:{z}, heart_eyed_cat:{heart_eyed_cat}");

    //Quiz
    // let x : u8 = 0;
    // lex x = x - 1;
    // println!("x:{x}");

    // let x: fsize = 2.0;

    // println!("{x}");

    //Compound Types
    //The Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("(x.0:{five_hundred}, x.1:{six_point_four}, x.2:{one})");

    let mut x: (i32, i32) = (1, 2);
    x.0 = 0;
    x.1 += 5;
    println!("x.0:{}", x.0);
    // how to print tuple element like Python way?
    // println!("x.0:{x.0}");
    println!("x:{x:?}");
    println!("x:{x:#?}");


    //The Array Type
    //vector in Chapter 8
    let a = [1, 2, 3, 4, 5];
    println!("a:{a:?}");
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("months:{months:?}");
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a:{a:?}");
    let a = [3; 5];
    println!("a:{a:?}");

    //Accessing Array Elements
    let a = [1, 2, 3, 4, 5];
    println!("a:{a:?}");

    let first = a[0];
    let second = a[1];
    println!("first:{first}");
    println!("second:{second}");

    //Invalid Array Element Access
    //Chapter 9 discusses more of Rust’s error handling and how you can write readable, safe code that neither panics nor allows invalid memory access.
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    println!("The value of the element at index {index} is: {element}");

    //Quiz
    //Question 1
    let message = "The temperature today is:";
    let x = [message, 100];
    println!("{} {}", x[0], x[1]);
    //Question 2
    let t = ([1; 2], [3; 4]);
    let (a, b) = t;
    println!("{}", a[0] + t.1[0]);
}
