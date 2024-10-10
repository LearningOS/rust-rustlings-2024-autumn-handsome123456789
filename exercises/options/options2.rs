// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.



#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // 使用 if let 语句来解构 Option 并断言其值
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = Vec::new();

        for i in 1..=range {
            optional_integers.push(Some(i as i8));
        }

        let mut cursor = range;

        // 使用 while let 语句来解构 Option 并断言其值
        while let Some(integer) = optional_integers.pop() {
            assert_eq!(integer, Some(cursor)); // 这里不需要使用 Some 包裹
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
