// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($input:expr) => {
        println!("Check out my macro with input: {}", $input);
    };
}
fn main() {
    my_macro!();
    my_macro!("Hello, Rust!");
}


