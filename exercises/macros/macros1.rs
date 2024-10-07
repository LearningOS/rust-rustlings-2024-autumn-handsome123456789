// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

macro_rules! my_macro {
    ($input:expr) => {
        println!("Check out my macro with input: {}", $input);
    };
}

fn main() {
    my_macro!("Hello, Rust!");
}
