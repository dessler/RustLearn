// 主入口
use std::io;
mod multiplication_tables;
mod arithmetic_operations;

use multiplication_tables::{generate_99_table, generate_88_table};

fn main(){
    loop {
        println!("请选择要做的操作");
        println!("1. 打印乘法表");
        println!("2. 进行加减乘除四则运算");
        println!("3. 退出整个程序");
        let mut user_choice = String::new();
        io::stdin().read_line(&mut user_choice).expect("无法读取输入");

        let user_choice: u32 = match user_choice.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("请输入有效的选项（1或者2或者3）");
                continue;
            }
        };
        match user_choice{
            1 => {
                println!("输出99乘法表");
                generate_99_table();
            }
            2 => {
                println!("输出88乘法表");
                generate_88_table();
            }
            3 => {
                println!("退出程序");
                break;
            }
            _ =>{
                println!("请输出有效的选项");
                continue;
            }
        }
    }
}

loop {
println!("请选择要输出的乘法表");
println!("1. 99乘法表");
println!("2. 88乘法表");
println!("3. 退出");
let mut user_choice = String::new();
io::stdin().read_line(&mut user_choice).expect("无法读取输入");

let user_choice: u32 = match user_choice.trim().parse(){
Ok(num) => num,
Err(_) => {
println!("请输入有效的选项（1或者2或者3）");
continue;
}
};
match user_choice{
1 => {
println!("输出99乘法表");
generate_99_table();
}
2 => {
println!("输出88乘法表");
generate_88_table();
}
3 => {
println!("退出程序");
break;
}
_ =>{
println!("请输出有效的选项");
continue;
}
}
}

// 知识点
// loop 是一个无限循环的函数，只要未选择退出，就会一直循环
// mod 导入模块，可以是同目下对应名字文件，也可以是对应目录下的mod.rs文件
// let 设置了一个可变变量，采用string::new 设置了一个字符串
// io::stdin 获取标准输入，存入变量中，并声明这个变量是可变的
// 对用户的输入进行处理(u32转为无符号整形，trim() 去除两端空格，parse（）解析为数字)
// 如果解析失败则抛出异常，提示只能输出数字1，2，3，如果解析成功则进行后续处理（只有数字才会进行后续）。
// 后续根据解析成功的数字（只有是数字才会到这一步），执行对应的的函数或者执行退出，如果没匹配上，则抛出异常，让用户重新选。