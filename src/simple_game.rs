use std::io;

pub fn play_simple_game() {
    println!("🎮 简单测试游戏");
    println!("这是一个测试游戏，用来验证输入输出是否正常工作。");
    
    loop {
        println!("\n请选择一个选项:");
        println!("1. 打招呼");
        println!("2. 显示当前时间信息");
        println!("3. 退出");
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let choice = input.trim();
                println!("你输入了: '{}'", choice);
                
                match choice {
                    "1" => println!("👋 你好！欢迎使用Rust游戏！"),
                    "2" => println!("⏰ 这是一个简单的Rust控制台程序"),
                    "3" => {
                        println!("👋 再见！");
                        break;
                    }
                    _ => println!("❌ 无效选择: '{}', 请输入1、2或3", choice),
                }
            }
            Err(e) => {
                println!("❌ 读取输入时出错: {}", e);
                break;
            }
        }
    }
}