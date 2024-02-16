fn main() {
    let (a, b) = read_ab();
    let d = a / b;
    let r = a % b;
    let f = a as f64 / b as f64;

    println!("{} {} {:.5}", d, r, f);
}

fn read_ab() -> (i32, i32) {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let ab: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    (ab[0], ab[1])
}
