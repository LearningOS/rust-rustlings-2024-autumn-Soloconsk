// errors1.rs
//
// 这个函数在传入空字符串时拒绝生成名牌文本。如果它能解释问题所在，而不是
// 仅仅返回 `None`，那就更好了。幸运的是，Rust 有一个类似于 `Result` 的
// 结构可以用来表达错误条件。让我们来使用它！
//
// 执行 `rustlings hint errors1` 或使用 `hint` watch 子命令获取提示。

pub fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        // 空名字是不允许的。
        Err("`name` was empty; it must be nonempty.".into())
    } else {
        Ok(format!("Hi! My name is {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // 不要改变这一行
            Err("`name` was empty; it must be nonempty.".into())
        );
    }
}
