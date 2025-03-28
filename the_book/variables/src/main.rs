const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
const TWO: u32 = 1 + 1;
fn main() {
    //Variables and Mutability
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    //Constants
    println!("THREE_HOURS_IN_SECONDS:{THREE_HOURS_IN_SECONDS}");
    println!("TWO:{TWO}");
    //Shadowing
    let y = 60;
    let y = y + 9;
    {
        let y = y * 2;
        println!("The value of y in the innner scope is: {y}");
    }
    println!("The value of y is: {y}");
    let spaces = "   ";
    let spaces = spaces.len();
    println!("string len:{spaces}");
    // let mut spaces = "   ";
    // spaces = spaces.len();
    // println!("string len:{spaces}");
}