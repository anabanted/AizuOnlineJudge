fn main() {
    let mut x = String::new();
    std::io::stdin().read_line(&mut x).unwrap();
    let x: i32 = x.trim().parse().unwrap();

    let cube_x = x.pow(3);

    println!("{}", cube_x);
}
