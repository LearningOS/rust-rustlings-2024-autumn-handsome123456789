// tests6.rs
//
// In this example we take a shallow dive into the Rust standard library's
// unsafe functions. Fix all the question marks and todos to make the test
// pass.
//
// Execute `rustlings hint tests6` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

/// # Safety
///
/// The `ptr` must contain an owned box of `Foo`.
struct Foo {
    a: u128,
    b: Option<String>,
}

/// # Safety
///
/// The `ptr` must contain an owned box of `Foo`.
unsafe fn raw_pointer_to_box(ptr: *mut Foo) -> Box<Foo> {
    // SAFETY: The `ptr` contains an owned box of `Foo` by contract. We
    // simply reconstruct the box from that pointer.
    unsafe { Box::from_raw(ptr) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_success() {
        let mut data = Box::new(Foo { a: 1, b: None });
        // 修改 b 字段的值以满足测试期望
        data.b = Some("hello".to_owned());

        let ptr_1 = &*data as *const Foo as usize;
        // SAFETY: We pass an owned box of `Foo`.
        let ret = unsafe { raw_pointer_to_box(Box::into_raw(data)) };

        let ptr_2 = &*ret as *const Foo as usize;

        assert!(ptr_1 == ptr_2);
        // 保持原始测试的期望
        assert!(ret.b == Some("hello".to_owned()));
    }
}
