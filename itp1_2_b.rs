fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    let abc: Vec<i32> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();

    if abc[0] < abc[1] && abc[1] < abc[2] {
        println!("Yes");
    } else {
        println!("No");
    }
}
