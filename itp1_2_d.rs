fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let v: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let (W, H, x, y, r) = (v[0], v[1], v[2], v[3], v[4]);

    match (x, y) {
        (x, y) if r <= x && x <= W - r && r <= y && y <= H - r => println!("Yes"),
        (_, _) => println!("No"),
    }
}
