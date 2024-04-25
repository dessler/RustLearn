//  四则运算函数
pub fn calculate(expression: &str) -> Option<f64> {
    let parts: Vec<&str> = expression.trim().split_whitespace().collect();

    if parts.len() != 3 {
        println!("输入的表达式不正确，请输入两个数字和一个操作符，例如1+2");
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