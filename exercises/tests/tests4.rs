// tests4.rs
//
// 确保我们正在测试正确的条件！
//
// 执行 `rustlings hint tests4` 或使用 `hint` watch 子命令获取提示。

struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {
    // 只修改测试函数本身
    pub fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            panic!("Rectangle width and height cannot be negative!")
        }
        Rectangle {width, height}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // 这个测试应该检查矩形的大小是否与我们传入构造函数的参数相符
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10); // 检查宽度
        assert_eq!(rect.height, 20); // 检查高度
    }

    #[test]
    #[should_panic]
    fn negative_width() {
        // 这个测试应该检查当我们尝试创建一个负宽度的矩形时程序是否会panic
        let _rect = Rectangle::new(-10, 10);
    }

    #[test]
    #[should_panic]
    fn negative_height() {
        // 这个测试应该检查当我们尝试创建一个负高度的矩形时程序是否会panic
        let _rect = Rectangle::new(10, -10);
    }
}
