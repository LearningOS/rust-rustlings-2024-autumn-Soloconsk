// tests7.rs
//
// 在构建包时，某些依赖项既不能在 `Cargo.toml` 中导入，也不能直接链接；
// 一些预处理过程从代码生成到设置包特定的配置都有所不同。
//
// Cargo 并不旨在取代其他构建工具，但它通过名为 `build.rs` 的自定义构建脚本
// 与它们集成。这个文件通常放在项目的根目录，而在这个例子中，它位于本练习的
// 同一目录下。
//
// 它可以用于：
//
// - 构建捆绑的 C 库。
// - 在主机系统上查找 C 库。
// - 从规范生成 Rust 模块。
// - 执行 crate 所需的任何特定于平台的配置。
//
// 在设置配置时，我们可以在构建脚本中使用 `println!` 来告诉 Cargo 遵循一些指令。
// 通用格式是：
//
//     println!("cargo:{}", your_command_in_string);
//
// 请查看 Cargo 官方书籍中关于构建脚本的更多信息：
// https://doc.rust-lang.org/cargo/reference/build-scripts.html
//
// 在这个练习中，我们查找一个环境变量并期望它落在一个范围内。
// 你可以查看测试用例以了解详细信息。
//
// 你不应该修改这个文件。修改同一目录下的 `build.rs` 以通过这个练习。
//
// 执行 `rustlings hint tests7` 或使用 `hint` watch 子命令获取提示。

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let s = std::env::var("TEST_FOO").unwrap();
        let e: u64 = s.parse().unwrap();
        assert!(timestamp >= e && timestamp < e + 10);
    }
}
