fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let vec: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let a = vec[0];
    let b = vec[1];

    println!("{} {}", area(&a, &b), perimeter(&a, &b));
}

fn area(a: &i32, b: &i32) -> i32 {
    return a * b;
}

fn perimeter(a: &i32, b: &i32) -> i32 {
    return 2 * (a + b);
}
