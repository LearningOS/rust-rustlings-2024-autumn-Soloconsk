// tests9.rs
//
// Rust能够与C/C++和其他静态编译语言共享FFI接口，甚至可以在代码内部进行链接！这是通过extern块实现的，就像下面的代码一样。
//
// extern关键字后的短字符串表示外部导入函数将遵循的ABI。在这个练习中，使用了"Rust"，而其他变体如"C"用于标准C ABI，"stdcall"用于Windows ABI。
//
// 外部导入的函数在extern块中声明，使用分号标记签名的结束，而不是大括号。可以对这些函数声明应用一些属性来修改链接行为，例如#[link_name = ".."]来修改实际的符号名称。
//
// 如果你想将你的符号导出到链接环境，也可以在函数定义前标记extern关键字，并带有相同的ABI字符串注释。Rust函数的默认ABI实际上就是"Rust"，所以如果你想链接纯Rust函数，可以省略整个extern术语。
//
// Rust默认会对符号进行名称修饰，就像C++一样。要抑制这种行为并使这些函数可以通过名称寻址，可以应用#[no_mangle]属性。
//
// 在这个练习中，你的任务是使测试用例能够调用Foo模块中的my_demo_function。my_demo_function_alias是my_demo_function的别名，所以测试用例中的两行代码应该调用相同的函数。
//
// 你不应该修改任何现有代码，只需添加两行属性。

extern "Rust" {
    fn my_demo_function(a: u32) -> u32;
    #[link_name = "my_demo_function"]
    fn my_demo_function_alias(a: u32) -> u32;
}

mod foo {
    // 没有`extern`等同于`extern "Rust"`。
    #[no_mangle]
    pub extern "Rust" fn my_demo_function(a: u32) -> u32 {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // 默认情况下，外部导入的函数是不安全的，
        // 因为它们来自其他语言的不受信任的源。你可以
        // 将它们包装在安全的Rust API中，以减轻调用者的负担。
        //
        // 安全性：我们知道这些函数是安全的Rust函数的别名。
        unsafe {
            my_demo_function(123);
            my_demo_function_alias(456);
        }
    }
}
