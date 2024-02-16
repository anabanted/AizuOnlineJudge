fn main() {
    loop {
        let (a, b, op) = read_input();
        let result = calculate(a, b, op);
        match result {
            Result::Value(v) => println!("{}", v),
            Result::End => break,
        }
    }
}

fn read_input() -> (i32, i32, Operator) {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let v: Vec<&str> = input.trim().split_whitespace().collect();

    let a: i32 = v[0].parse().unwrap();
    let b: i32 = v[2].parse().unwrap();

    let op = match &*v[1] {
        "+" => Operator::Add,
        "-" => Operator::Sub,
        "*" => Operator::Mul,
        "/" => Operator::Div,
        "?" => Operator::End,
        _ => panic!("Invalid operator"),
    };

    (a, b, op)
}

fn calculate(a: i32, b: i32, op: Operator) -> Result {
    match op {
        Operator::Add => Result::Value(a + b),
        Operator::Sub => Result::Value(a - b),
        Operator::Mul => Result::Value(a * b),
        Operator::Div => Result::Value(a / b),
        Operator::End => Result::End,
    }
}

enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    End,
}

enum Result {
    Value(i32),
    End,
}
