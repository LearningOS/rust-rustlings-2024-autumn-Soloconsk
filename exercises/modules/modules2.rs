// modules2.rs
//
// 你可以使用 'use' 和 'as' 关键字将模块路径引入作用域并为它们提供新的名称。
// 修复这些 'use' 语句以使代码编译通过。
//
// 执行 `rustlings hint modules2` 或使用 `hint` watch 子命令获取提示。

mod delicious_snacks {
    // 修复这些 use 语句
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;

    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    println!(
        "最喜欢的零食：{} 和 {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}
