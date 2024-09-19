// lifetimes3.rs
//
// 结构体中的引用也需要生命周期。
//
// 执行 `rustlings hint lifetimes3` 或使用 `hint` watch 子命令获取提示。

struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}
