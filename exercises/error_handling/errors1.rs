// errors1.rs
// This function refuses to generate text to be printed on a nametag if
// you pass it an empty string. It'd be nicer if it explained what the problem
// was, instead of just sometimes returning `None`. The 2nd test currently
// does not compile or pass, but it illustrates the behavior we would like
// this function to have.
// Execute `rustlings hint errors1` for hints!


// pub fn generate_nametag_text(name: String) -> Result<String, &'static str> {
//     if name.len() > 0 {
//         Ok(format!("Hi! My name is {}", name))
//     } else {
//         // Empty names aren't allowed.
//         Err("`name` was empty; it must be nonempty.")
//     }
// }

pub fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.len() > 0 {
        Ok(format!("Hi! My name is {}", name))
    } else {
        // Empty names aren't allowed.
        Err(String::from("`name` was empty; it must be nonempty."))
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    // This test passes initially if you comment out the 2nd test.
    // You'll need to update what this test expects when you change
    // the function under test!
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
