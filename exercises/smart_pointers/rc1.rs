// rc1.rs
//
// 在这个练习中，我们想通过 Rc<T> 类型来表达多个所有者的概念。这是我们太阳系的一个模型 - 
// 有一个 Sun 类型和多个 Planet。Planet 拥有 sun 的所有权，表示它们围绕太阳运转。
//
// 通过使用适当的 Rc 原语来表达太阳有多个所有者，使这段代码能够编译。
//
// 执行 `rustlings hint rc1` 或使用 `hint` watch 子命令获取提示。

use std::rc::Rc;

#[derive(Debug)]
struct Sun {}

#[derive(Debug)]
enum Planet {
    Mercury(Rc<Sun>),
    Venus(Rc<Sun>),
    Earth(Rc<Sun>),
    Mars(Rc<Sun>),
    Jupiter(Rc<Sun>),
    Saturn(Rc<Sun>),
    Uranus(Rc<Sun>),
    Neptune(Rc<Sun>),
}

impl Planet {
    fn details(&self) {
        println!("你好，来自 {:?}!", self)
    }
}

fn main() {
    let sun = Rc::new(Sun {});
    println!("引用计数 = {}", Rc::strong_count(&sun)); // 1 个引用

    let mercury = Planet::Mercury(Rc::clone(&sun));
    println!("引用计数 = {}", Rc::strong_count(&sun)); // 2 个引用
    mercury.details();

    let venus = Planet::Venus(Rc::clone(&sun));
    println!("引用计数 = {}", Rc::strong_count(&sun)); // 3 个引用
    venus.details();

    let earth = Planet::Earth(Rc::clone(&sun));
    println!("引用计数 = {}", Rc::strong_count(&sun)); // 4 个引用
    earth.details();

    let mars = Planet::Mars(Rc::clone(&sun));
    println!("引用计数 = {}", Rc::strong_count(&sun)); // 5 个引用
    mars.details();

    let jupiter = Planet::Jupiter(Rc::clone(&sun));
    println!("引用计数 = {}", Rc::strong_count(&sun)); // 6 个引用
    jupiter.details();

    let saturn = Planet::Saturn(Rc::clone(&sun));
    println!("引用计数 = {}", Rc::strong_count(&sun)); // 7 个引用
    saturn.details();

    let uranus = Planet::Uranus(Rc::clone(&sun));
    println!("引用计数 = {}", Rc::strong_count(&sun)); // 8 个引用
    uranus.details();

    let neptune = Planet::Neptune(Rc::clone(&sun));
    println!("引用计数 = {}", Rc::strong_count(&sun)); // 9 个引用
    neptune.details();

    assert_eq!(Rc::strong_count(&sun), 9);

    drop(neptune);
    println!("引用计数 = {}", Rc::strong_count(&sun)); // 8 个引用

    drop(uranus);
    println!("引用计数 = {}", Rc::strong_count(&sun)); // 7 个引用

    drop(saturn);
    println!("引用计数 = {}", Rc::strong_count(&sun)); // 6 个引用

    drop(jupiter);
    println!("引用计数 = {}", Rc::strong_count(&sun)); // 5 个引用

    drop(mars);
    println!("引用计数 = {}", Rc::strong_count(&sun)); // 4 个引用

    drop(earth);
    println!("引用计数 = {}", Rc::strong_count(&sun)); // 3 个引用

    drop(venus);
    println!("引用计数 = {}", Rc::strong_count(&sun)); // 2 个引用

    drop(mercury);
    println!("引用计数 = {}", Rc::strong_count(&sun)); // 1 个引用

    assert_eq!(Rc::strong_count(&sun), 1);
}
