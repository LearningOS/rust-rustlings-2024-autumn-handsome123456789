// primitive_types4.rs
//
// Get a slice out of Array a where the ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand
// for a hint.


#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    // 切片从索引 1 开始到索引 4 结束（不包括索引 4）
    let nice_slice = &a[1..4];

    // 断言切片 nice_slice 包含元素 [2, 3, 4]
    assert_eq!([2, 3, 4], nice_slice);
}
