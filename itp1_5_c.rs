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
    for row in 0..h {
        match (row % 2, w % 2) {
            (0, 0) => println!("{}", "#.".repeat((w / 2) as usize)),
            (0, 1) => println!("{}", "#.".repeat((w / 2) as usize) + "#"),
            (1, 0) => println!("{}", ".#".repeat((w / 2) as usize)),
            (1, 1) => println!("{}", ".#".repeat((w / 2) as usize) + "."),
            (_, _) => panic!("unexpected"),
        }
    }

    println!();
}
