// errors1.rs
//
// This function refuses to generate text to be printed on a nametag if you pass
// it an empty string. It'd be nicer if it explained what the problem was,
// instead of just sometimes returning `None`. Thankfully, Rust has a similar
// construct to `Result` that can be used to express error conditions. Let's use
// it!
//
// Execute `rustlings hint errors1` or use the `hint` watch subcommand for a
// hint.



pub fn generate_nametag_text(name: String) -> Result<String, String> {
    // This function should return a Result<String, String> instead of an
    // Option<String>. If the name is empty, it should return an error with a
    // message explaining that the name was empty. Otherwise, it should return
    // the string "Hi! My name is <name>".
    if name.is_empty() {
        // Empty names aren't allowed.
        Result::Err("`name` was empty; it must be nonempty.".into())
    } else {
        Result::Ok(format!("Hi! My name is {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // Don't change this line
            Err("`name` was empty; it must be nonempty.".into())
        );
    }
}
