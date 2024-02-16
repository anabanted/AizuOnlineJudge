fn main() {
    let mut i = 1;
    loop {
        let n = read_input();
        if n == 0 {
            break;
        }
        print_case(i, n);
        i += 1;
    }
}

fn print_case(i: u32, n: i32) {
    println!("Case {}: {}", i, n);
}

fn read_input() -> i32 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let n: i32 = s.trim().parse().unwrap();
    n
}
