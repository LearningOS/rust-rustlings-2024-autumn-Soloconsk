// modules3.rs
//
// 你可以使用 'use' 关键字将模块路径从任何地方，特别是从 Rust 标准库中引入到你的作用域。
// 从 std::time 模块中引入 SystemTime 和 UNIX_EPOCH。如果你能用一行完成，那就更好了！
//
// 执行 `rustlings hint modules3` 或使用 `hint` watch 子命令获取提示。

// 我还没有完成

// TODO: 完成这个 use 语句
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC 是 {} 秒之前!", n.as_secs()),
        Err(_) => panic!("SystemTime 在 UNIX EPOCH 之前!"),
    }
}
