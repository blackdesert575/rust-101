use std::any::type_name;
// use std::process::exit;

fn print_type<T>(_: &T) {
    println!("Type: {}", type_name::<T>());
}

// Control Flow
//if Expressions
fn main() {
    let number = 7;

    if number < 5 {
        println!("1st test condition was true");
    }
    else {
        println!("condition was false");
    }

    
// // If you don’t provide an else expression and the condition is false, 
// // the program will just skip the if block and move on to the next bit of code.
//     let number = 10;

//     if number < 5 {
//         println!("condition was true");
//     }
//     // else {
//     //     println!("condition was false");
//     // }


    // let number = 3;
    // // It’s also worth noting that the condition in this code must be a bool. If the condition isn’t a bool, we’ll get an error.
    // if number {
    //     println!("number was three");
    // }
/*     The error indicates that Rust expected a bool but got an integer. 
    Unlike languages such as Ruby and JavaScript, 
    Rust will not automatically try to convert non-Boolean types to a Boolean. */

    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }

    //Handling Multiple Conditions with else if
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    //That’s because Rust only executes the block for the first true condition, and once it finds one, it doesn’t even check the rest.
    //Using too many else if expressions can clutter your code, 
    //so if you have more than one, you might want to refactor your code.
    //Chapter 6 describes a powerful Rust branching construct called match for these cases.

    //Using if in a let Statement
    let condition = true;
    //error[E0308]: `if` and `else` have incompatible types
    // let number = if condition { 0.99 } else { 6 };
    let number: u32 = if condition { 7 } else { 6 };

    print_type(&number);  // 檢查 number 的類型
    println!("The value of number is: {number}");

    // let condition = true;

    // let number = if condition { 5 } else { "six" };

    // println!("The value of number is: {number}");

    //Quiz
    // //Question 1

    // // (Note: both of these snippets compile!)
    // // let cond = true;
    // // let cond = false;

    // let x = if cond { 1 } else { 2 };
    // println!("{}",x);

    // let x;
    // if cond {
    // x = 1;
    // } else {
    // x = 2;
    // }
    // println!("{}",x);

    //Question 2
    // let x = 1;
    // let y = if x { 0 } else { 1 }; 
    // println!("{y}");


    //Repetition with Loops
    // loop
    // loop {
    //     println!("again!");
    // }

    //Returning Values from Loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
            // return counter * 2;
        }
    };
    // return counter * 2;
    println!("The result is {result}");

    //Loop Labels to Disambiguate Between Multiple Loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    //Conditional Loops with while
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    //Looping Through a Collection with for
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

        let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    //rev, to reverse the range:
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    //Quiz
    //Question 1

    let mut x = 0;
    'a: loop {
        x += 1;
        'b: loop {
            if x > 10 {
                continue 'a;
            } else {
                break 'b;
            }      
        }
        break;       
    }
    // exit;
    //Question 2

    let a = [5; 10];
    let mut sum = 0;
    for x in a {
        sum += x;
    }
    println!("{sum}");    
}
