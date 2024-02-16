fn main() {
    let n = read_n();
    for i in 1..=n {
        if can_devide_three(&i) || include_three(&i) {
            print!(" {}", i);
        }
    }
}

fn read_n() -> u32 {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    n.trim().parse().unwrap()
}

fn can_devide_three(n: &u32) -> bool {
    match n % 3 {
        0 => true,
        _ => false,
    }
}

fn include_three(n: &u32) -> bool {
    let mut n = *n;
    while n > 0 {
        if n % 10 == 3 {
            return true;
        }
        n /= 10;
    }
    false
}
