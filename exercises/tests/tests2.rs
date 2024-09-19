// tests2.rs
//
// 这个测试有一个问题 -- 让测试能够编译！让测试通过！让测试失败！
//
// 执行 `rustlings hint tests2` 或使用 `hint` watch 子命令获取提示。

#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        assert_eq!(2 + 2, 4);
    }
}
