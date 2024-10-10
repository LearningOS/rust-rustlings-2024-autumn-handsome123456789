// tests8.rs
//
// This execrise shares `build.rs` with the previous exercise.
// You need to add some code to `build.rs` to make both this exercise and
// the previous one work.
//
// Execute `rustlings hint tests8` or use the `hint` watch subcommand for a
// hint.



// tests8.rs


#[cfg(test)]
mod tests {
    #[test]
    fn test_success() {
        #[cfg(feature = "pass")]
        return; // 如果定义了 `pass` 特性，测试直接通过

        #[cfg(not(feature = "pass"))]
        panic!("no cfg set"); // 如果没有定义 `pass` 特性，测试失败
    }
}