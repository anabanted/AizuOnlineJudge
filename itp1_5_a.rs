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
    for _ in 0..h {
        let line = "#".repeat(w as usize);
        println!("{}", line);
    }

    println!();
}
