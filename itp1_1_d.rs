fn main() {
    let mut s_input = String::new();
    std::io::stdin().read_line(&mut s_input).unwrap();
    let s_input: i32 = s_input.trim().parse().unwrap();

    let h = s_input / 3600;
    let m = (s_input % 3600) / 60;
    let s = s_input % 60;

    println!("{}:{}:{}", h, m, s);
}
