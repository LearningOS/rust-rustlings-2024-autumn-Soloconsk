// threads1.rs
//
// 这个程序生成多个线程，每个线程至少运行250毫秒，并且每个线程返回它们完成所需的时间。
// 程序应该等待所有生成的线程完成，并将它们的返回值收集到一个向量中。
//
// 执行 `rustlings hint threads1` 或使用 `hint` watch 子命令获取提示。

use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles = vec![];
    for i in 0..10 {
        handles.push(thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("线程 {} 已完成", i);
            start.elapsed().as_millis()
        }));
    }

    let mut results: Vec<u128> = vec![];
    for handle in handles {
        // 使用 join() 方法等待线程完成并获取其返回值
        results.push(handle.join().unwrap());
    }

    if results.len() != 10 {
        panic!("哦不！并非所有生成的线程都完成了！");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("线程 {} 耗时 {}毫秒", i, result);
    }
}
