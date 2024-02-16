fn main() {
    let (_n, v) = read_input();

    let min = v.iter().min().unwrap();
    let max = v.iter().max().unwrap();
    let sum = v.iter().sum::<i64>();

    println!("{} {} {}", min, max, sum);
}

fn read_input() -> (i64, Vec<i64>) {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let n: i64 = n.trim().parse().unwrap();

    let mut v = String::new();
    std::io::stdin().read_line(&mut v).unwrap();
    let v: Vec<i64> = v.split_whitespace().map(|x| x.parse().unwrap()).collect();

    (n, v)
}
