fn main() {
    let a = reverse_vector(read_input().unwrap());
    println!(
        "{}",
        a.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

fn read_input() -> Option<Vec<i32>> {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<i32>().unwrap();

    let mut a = String::new();
    std::io::stdin().read_line(&mut a).unwrap();
    let a = a
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    if a.len() == n as usize {
        Some(a)
    } else {
        None
    }
}

fn reverse_vector(a: Vec<i32>) -> Vec<i32> {
    let mut b = a.clone();
    b.reverse();
    b
}
