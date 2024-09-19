// tests3.rs
//
// 这个测试没有测试我们的函数 -- 以一种使测试通过的方式来测试它。
// 然后编写第二个测试，测试我们调用 `is_even(5)` 时是否得到预期的结果。
//
// 执行 `rustlings hint tests3` 或使用 `hint` watch 子命令获取提示。

pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!(is_even(4));
    }

    #[test]
    fn is_false_when_odd() {
        assert!(!is_even(5));
    }
}
