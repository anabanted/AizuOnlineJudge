use std::cmp;
use std::io::{Error, ErrorKind};

fn main() {
    loop {
        let (n, x) = read_input().unwrap();
        match (n, x) {
            (0, 0) => break,
            (n, x) => match solve(n, x) {
                Ok(count) => println!("{}", count),
                Err(e) => panic!("{}", e),
            },
        }
    }
}

fn read_input() -> Result<(i32, i32), Error> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let nx: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    match nx.len() {
        2 => Ok((nx[0], nx[1])),
        _ => Err(Error::new(ErrorKind::InvalidInput, "invalid input")),
    }
}

fn solve(n: i32, x: i32) -> Result<i32, Error> {
    if x > 3 * n - 3 {
        return Ok(0);
    }
    let min = cmp::max(1, x - 2 * n + 1);
    let max = x / 3 - 1;
    sum_by_a(min, max, x, n)
}

fn count_by_a(a: i32, x: i32, n: i32) -> i32 {
    cmp::max((x - a - 1) / 2 - cmp::max(a + 1, x - n - a) + 1, 0)
}

fn sum_by_a(start: i32, end: i32, x: i32, n: i32) -> Result<i32, Error> {
    if start > end {
        return Ok(0);
    }
    let mut sum = 0;
    for a in start..=end {
        sum += count_by_a(a, x, n);
    }
    Ok(sum)
}
