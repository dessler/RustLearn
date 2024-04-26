//  四则运算函数
pub fn calculate(expression: &str) -> Option<f64> {
    let parts: Vec<&str> = expression.trim().split_whitespace().collect();

    if parts.len() != 3 {
        println!("输入的表达式不正确，请输入两个数字和一个操作符，例如1 + 2 ，必须包含空格，实际这里是可以用代码来实现修复这个问题的");
        return None;
    }

    let num1: f64 = match parts[0].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("第一个数字无效");
            return None;
        }
    };
    let operator = parts[1];

    let num2: f64 = match parts[2].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("第二个数字无效");
            return None;
        }
    };

    match operator {
        "+" => Some(num1 + num2),
        "-" => Some(num1 - num2),
        "*" => Some(num1 * num2),
        "/" => {
            if num2 == 0.0 {
                println!("除数不能为零");
                return None;
            } else {
                Some(num1 / num2)
            }
        },
        _ => {
            println!("无效的操作符");
            return None;
        }
    }
}

// 知识点
// 定义公共函数，需要接受其他expression变量的输入，并将收到的值进行了一些预处理
// 对数字变量进行了变量类型的改变
// 使用match方式对符号做不同的判断