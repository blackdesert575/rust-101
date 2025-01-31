use std::any::type_name;
fn another_function() {
    println!("Another function.");
}

fn another_function_with_a_parameter(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// if this is a parameter name, give it a type
// fn f(x) { 
//   println!("{x}");
// }

fn print_type<T>(_: &T) {
    println!("Type: {}", type_name::<T>());
}

// rustc --explain E0308
// fn five() -> i32 {
//     return 5;
// }

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn f(x: i32) -> i32 { x + 1 }

fn main() {
    println!("Hello, world!");
    //Functions
    another_function();
    //Parameters
    another_function_with_a_parameter(5);
    print_labeled_measurement(5, 'h');
    // f(0);
    //Statements and Expressions
    // let y = 6;
    // let x = (let y = 6);
    // expression with {}
    let y = {
        let x = 3;
        x + 1
        // x + 1;
        // rustc --explain E0277
        //Expressions do not include ending semicolons. 
        //If you add a semicolon to the end of an expression, you turn it into a statement, 
        //and it will then not return a value.
        //Keep this in mind as you explore function return values and expressions next.        
    };

    println!("The value of y is: {y}");

    // block expression
    // "{}"
    let y = {4};
    println!("The value of y is: {y}");
    
    print_type(&y);  // 檢查 y 的類型

    //Functions with Return Values
    // example 01
    let x = five();
    println!("The value of x is: {x}");

    // example 02
    let x = plus_one(5);
    
    println!("The value of x is: {x}");

    // Quiz
    // Question 1
    // An expression
    // A syntactic scope
    { /* ... */ }

    //Question 2
    println!("{}", f({
        let y = 1;
        y + 1
    }));    
}