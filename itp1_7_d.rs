use std::fmt;
use std::ops::Mul;

fn main() {
    let (n, m, l) = read_nml();
    let a = read_matrix(n, m).unwrap();
    let b = read_matrix(m, l).unwrap();
    let c = a * b;
    println!("{}", c);
}

fn read_nml() -> (usize, usize, usize) {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut ws = buf.split_whitespace();
    let n: usize = ws.next().unwrap().parse().unwrap();
    let m: usize = ws.next().unwrap().parse().unwrap();
    let l: usize = ws.next().unwrap().parse().unwrap();
    (n, m, l)
}

fn read_matrix(n: usize, m: usize) -> Option<Matrix> {
    let mut data = Vec::new();
    for _ in 0..n {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let row: Vec<i64> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();
        if row.len() != m {
            return None;
        }
        data.push(row);
    }
    Some(Matrix { data })
}

struct Matrix {
    data: Vec<Vec<i64>>,
}

impl Mul for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut data = Vec::new();
        for i in 0..self.data.len() {
            let left_row = &self.data[i];
            let mut row = Vec::new();
            for k in 0..rhs.data[0].len() {
                let sum = left_row
                    .iter()
                    .zip(rhs.data.iter().map(|r| r[k]))
                    .map(|(a, b)| a * b)
                    .sum();
                row.push(sum);
            }
            data.push(row);
        }
        Matrix { data }
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self
            .data
            .iter()
            .map(|row| {
                row.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", s)
    }
}
