// as_ref_mut.rs
//
// AsRef和AsMut允许进行廉价的引用到引用的转换。更多信息请阅读
// https://doc.rust-lang.org/std/convert/trait.AsRef.html 和
// https://doc.rust-lang.org/std/convert/trait.AsMut.html。
//
// 执行 `rustlings hint as_ref_mut` 或使用 `hint` watch 子命令获取提示。

// 获取给定参数的字节数（不是字符数）。
// 添加AsRef trait作为适当的trait约束。
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().as_bytes().len()
}

// 获取给定参数的字符数（不是字节数）。
// 添加AsRef trait作为适当的trait约束。
fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// 使用as_mut()对数字进行平方。
// 添加适当的trait约束。
fn num_sq<T: AsMut<u32>>(arg: &mut T) {
    // 实现函数体。
    let num = arg.as_mut();
    *num = *num * *num;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mult_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
