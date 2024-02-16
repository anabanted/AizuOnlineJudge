fn main() {
    loop {
        let (h, w) = read_input();
        match (h, w) {
            (0, 0) => break,
            _ => print_rectangle(h, w),
        }
    }
}

fn read_input() -> (u32, u32) {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let v: Vec<u32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    (v[0], v[1])
}

fn print_rectangle(h: u32, w: u32) {
    if h < 3 || w < 3 {
        panic!("h and w must be greater than 2");
    }

    println!("{}", "#".repeat(w as usize));
    for _ in 0..h - 2 {
        println!("{}{}{}", "#", ".".repeat((w - 2) as usize), "#");
    }
    println!("{}", "#".repeat(w as usize));

    println!();
}
