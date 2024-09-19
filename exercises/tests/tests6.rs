// tests6.rs
//
// 在这个例子中，我们浅入Rust标准库的不安全函数。修复所有问号和待办事项，使测试通过。
//
// 执行 `rustlings hint tests6` 或使用 `hint` watch 子命令获取提示。

struct Foo {
    a: u128,
    b: Option<String>,
}

/// # 安全性
///
/// `ptr` 必须包含一个拥有所有权的 `Foo` 的 Box。
unsafe fn raw_pointer_to_box(ptr: *mut Foo) -> Box<Foo> {
    // 安全性：根据约定，`ptr` 包含一个拥有所有权的 `Foo` 的 Box。
    // 我们只是从该指针重建 Box。
    let mut ret: Box<Foo> = unsafe { Box::from_raw(ptr) };
    ret.b = Some("hello".to_owned());
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_success() {
        let data = Box::new(Foo { a: 1, b: None });

        let ptr_1 = &data.a as *const u128 as usize;
        // 安全性：我们传递一个拥有所有权的 `Foo` 的 Box。
        let ret = unsafe { raw_pointer_to_box(Box::into_raw(data)) };

        let ptr_2 = &ret.a as *const u128 as usize;

        assert!(ptr_1 == ptr_2);
        assert!(ret.b == Some("hello".to_owned()));
    }
}
