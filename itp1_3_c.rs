fn main() {
    loop {
        let (a, b) = read_input();
        match (a, b) {
            (0, 0) => break,
            (a, b) => swap(a, b),
        }
    }
}

fn read_input() -> (i32, i32) {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let ab: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    (ab[0], ab[1])
}

fn swap(a: i32, b: i32) {
    match a.cmp(&b) {
        std::cmp::Ordering::Less => println!("{} {}", a, b),
        std::cmp::Ordering::Greater => println!("{} {}", b, a),
        std::cmp::Ordering::Equal => println!("{} {}", a, b),
    }
}
