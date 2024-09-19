// traits3.rs
//
// 你的任务是为两个结构实现Licensed trait，并让它们返回相同的信息，而不需要写两次相同的函数。
//
// 考虑一下你可以在Licensed trait中添加什么。
//
// 执行 `rustlings hint traits3` 或使用 `hint` watch 子命令获取提示。

pub trait Licensed {
    fn licensing_info(&self) -> String {
        String::from("Some information")
    }
}

struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}

impl Licensed for SomeSoftware {} // 不要编辑这行
impl Licensed for OtherSoftware {} // 不要编辑这行

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = String::from("Some information");
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), licensing_info);
        assert_eq!(other_software.licensing_info(), licensing_info);
    }
}
