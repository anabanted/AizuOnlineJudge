fn main() {
    let (a, b, c) = read_abc();

    let v = (a..=b).collect::<Vec<i32>>();

    let count = v
        .iter()
        .map(|x| can_devide(x, &c))
        .map(|x| count_devide(x))
        .sum::<i32>();

    println!("{}", count);
}

fn read_abc() -> (i32, i32, i32) {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let abc: Vec<i32> = s
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    (abc[0], abc[1], abc[2])
}

fn can_devide(d: &i32, n: &i32) -> DevideResult {
    match n % d {
        0 => DevideResult::Yes,
        _ => DevideResult::No,
    }
}

fn count_devide(d: DevideResult) -> i32 {
    match d {
        DevideResult::Yes => 1,
        DevideResult::No => 0,
    }
}

enum DevideResult {
    Yes,
    No,
}
