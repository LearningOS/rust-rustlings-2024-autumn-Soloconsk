//! 这是 tests7 和 tests8 的构建脚本。
//!
//! 你应该修改这个文件以使两个练习都通过。

fn main() {
    // 在 tests7 中，我们应该设置一个名为 `TEST_FOO` 的环境变量。
    // 在标准输出中打印以让 Cargo 执行此操作。
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let your_command = format!("rustc-env=TEST_FOO={}", timestamp);
    println!("cargo:{}", your_command);

    // 在 tests8 中，我们应该启用 "pass" 特性以使测试用例提前返回。
    // 填写命令以告诉 Cargo 这一点。
    let your_command = "rustc-cfg=feature=\"pass\"";
    println!("cargo:{}", your_command);
}
