// hashmaps1.rs
//
// 需要定义一个水果篮子，形式为哈希映射。键代表水果的名称，值代表该特定水果在篮子中的数量。
// 你必须在篮子中放入至少三种不同类型的水果（例如苹果、香蕉、芒果），所有水果的总数应至少为五个。
//
// 让我编译通过并通过测试！
//
// 执行 `rustlings hint hashmaps1` 或使用 `hint` watch 子命令获取提示。

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = HashMap::new(); // 声明你的哈希映射

    // 已经为你放入了两个香蕉 :)
    basket.insert(String::from("香蕉"), 2);

    // 在你的篮子里放入更多水果
    basket.insert(String::from("苹果"), 3);
    basket.insert(String::from("橙子"), 2);

    basket
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
