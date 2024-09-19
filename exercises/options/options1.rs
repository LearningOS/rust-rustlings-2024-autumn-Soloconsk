// options1.rs
//
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a
// hint.

// 这个函数返回冰箱里还剩多少冰淇淋。
// 如果在晚上10点之前，还剩5个。到了晚上10点，有人把它们都吃光了，所以就没有剩下的了 :(
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    // 我们在这里使用24小时制，所以晚上10点是22，凌晨12点是0
    // Option输出应该优雅地处理time_of_day > 23的情况。
    if time_of_day > 23 {
        None
    } else if time_of_day < 22 {
        Some(5)
    } else {
        Some(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // 修复这个测试。如何获取Option中包含的值？
        let icecreams = maybe_icecream(12);
        assert_eq!(icecreams.unwrap(), 5);
    }
}
