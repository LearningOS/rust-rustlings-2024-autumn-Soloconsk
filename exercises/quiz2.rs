// quiz2.rs
//
// 这是针对以下章节的测验：
// - 字符串
// - 向量
// - 移动语义
// - 模块
// - 枚举
//
// 让我们以函数的形式构建一个小机器。作为输入，我们将
// 给出一个字符串和命令的列表。这些命令决定了
// 将对字符串应用的操作。它可以是：
// - 将字符串转换为大写
// - 修剪字符串
// - 将"bar"附加到字符串上指定的次数
// 具体形式如下：
// - 输入将是一个2长度元组的向量，
//   第一个元素是字符串，第二个是命令。
// - 输出元素将是一个字符串的向量。
//
// 这次没有提示！

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            match command {
                Command::Uppercase => output.push(string.to_uppercase()),
                Command::Trim => output.push(string.trim().to_string()),
                Command::Append(n) => output.push(format!("{}{}", string, "bar".repeat(*n))),
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
