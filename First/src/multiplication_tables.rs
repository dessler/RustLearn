// 99乘法表
pub fn generate_99_table(){
    println!(" 99 乘法表");
    for i in 1..=9 {
        for j in 1..=i {
            let ij = i * j;
            print!("{:<2} * {:<2} = {:<4}", i, j, ij);
        }
    println!();
    }
}

// 88乘法表
pub fn  generate_88_table(){
    println!(" 88 乘法表");
    for i in 1..=8 {
        for j in 1..=i{
            let ij = i * j;
            print!("{:<2} * {:<2} = {:<4}", i ,j ,ij);
        }
    println!();
    }
}
// 知识点
// {:<2} 是一个格式化输出，  < 表示左对齐   2 表示2个字符，这样输出才是美观的
// println!() 是换行符
// 第二个变量j的范围基于第一个变量，这样可以保证显示更美观，否则就会输出一个正方形，不符合常理的三角形。