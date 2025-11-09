// TODO: This function refuses to generate text to be printed on a nametag if
// you pass it an empty string. It'd be nicer if it explained what the problem
// was instead of just returning `None`. Thankfully, Rust has a similar
// construct to `Option` that can be used to express error conditions. Change
// the function signature and body to return `Result<String, String>` instead
// of `Option<String>`.
fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        // Empty names aren't allowed
        Err("Empty names aren't allowed".to_string())
    } else {
        Ok(format!("Hi! My name is {name}"))
    }
}

fn main() {
    // You can optionally experiment here.
    // just print to stdout
    println!("case: 1");
    let input = String::from("");
    println!("{:#?}", generate_nametag_text(input));
    // with unwrap()
    println!("--------------------------------------");
    println!("case: 2");
    let ok_input = String::from("David");
    let result = generate_nametag_text(ok_input);
    println!("{:#?}", result);

    let err_input = String::new();
    let result = generate_nametag_text(err_input);
    print!("{:#?}", result);
    // with .expect("msg")
    println!("--------------------------------------");
    println!("case: 3");
    println!("remove comments to show case 3");
    // let ok_input = String::from("Asuka");
    // let result = generate_nametag_text(ok_input).expect("Name tag generation failed!");
    // println!("{:#?}", result);

    // let err_input = String::new();
    // let result = generate_nametag_text(err_input).expect("Expected a valid name, but got empty string!");
    // print!("{:#?}", result);
    // with match
    println!("--------------------------------------");
    println!("case: 4");
    let input = String::from("");
    match generate_nametag_text(input) {
        Ok(text) => println!("Generated tag: {text}"),
        Err(err) => println!("Failed: {err}"),
        
    }
    // with if let
    println!("--------------------------------------");
    println!("case: 5");
    let input = String::from("Rico");
    if let Ok(..) = generate_nametag_text(input) {
        println!("OK");
    } else {
        print!("Failed")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".to_string()).as_deref(),
            Ok("Hi! My name is Beyoncé"),
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text(String::new())
                .as_ref()
                .map_err(|e| e.as_str()),
            Err("Empty names aren't allowed"),
        );
    }
}
