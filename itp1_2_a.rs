fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let v: Vec<i32> = s
        .trim()
        .split_whitespace()
        .map(|e| e.parse().unwrap())
        .collect();

    let a = v[0];
    let b = v[1];

    let order = match a.cmp(&b) {
        std::cmp::Ordering::Less => Order::Ascending,
        std::cmp::Ordering::Greater => Order::Descending,
        std::cmp::Ordering::Equal => Order::Equal,
    };

    match order {
        Order::Ascending => println!("a < b"),
        Order::Descending => println!("a > b"),
        Order::Equal => println!("a == b"),
    }
}

enum Order {
    Ascending,
    Descending,
    Equal,
}
