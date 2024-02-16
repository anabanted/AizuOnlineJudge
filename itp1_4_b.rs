fn main() {
    let r = read_r();
    let pi = std::f64::consts::PI;

    let area = pi * r * r;
    let circumference = 2.0 * pi * r;

    println!("{} {}", area, circumference);
}

fn read_r() -> f64 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}
