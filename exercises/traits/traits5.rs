// traits5.rs
//
// Your task is to replace the '??' sections so the code compiles.
//
// Don't change any line other than the marked one.
//
// Execute `rustlings hint traits5` or use the `hint` watch subcommand for a
// hint.



pub trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

pub trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct {}
struct OtherStruct {}

impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// 定义一个新的 trait，它同时约束 SomeTrait 和 OtherTrait
pub trait CombinedTrait: SomeTrait + OtherTrait {}

// 为任何实现了 SomeTrait 和 OtherTrait 的类型实现 CombinedTrait
impl<T: SomeTrait + OtherTrait> CombinedTrait for T {}
// YOU MAY ONLY CHANGE THE NEXT LINE
fn some_func(item: &dyn CombinedTrait) -> bool {
    item.some_function() && item.other_function()
}

fn main() {
    some_func(&SomeStruct {});
    some_func(&OtherStruct {});
}
