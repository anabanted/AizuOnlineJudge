use std::ops::Mul;

fn main() {
    let (n, m) = read_size().unwrap();

    let matrix = read_matrix(n, m);
    let vector = read_vector(m);

    let result = matrix.mul(vector);
    print_vector(result);
}

fn read_size() -> Option<(usize, usize)> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let nm: Vec<usize> = buf
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    if nm.len() == 2 {
        Some((nm[0], nm[1]))
    } else {
        None
    }
}

fn read_line(m: usize) -> Option<Vec<i32>> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let v: Vec<i32> = buf
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    if v.len() == m {
        Some(v)
    } else {
        None
    }
}

fn read_matrix(n: usize, m: usize) -> Matrix<i32> {
    let mut data = Vec::new();
    for _ in 0..n {
        let line = read_line(m).unwrap();
        data.push(line);
    }
    Matrix { data }
}

fn read_vector(m: usize) -> Vec<i32> {
    let mut v = Vec::new();
    for _ in 0..m {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();

        v.push(buf.trim().parse().unwrap());
    }
    v
}

fn print_vector(v: Vec<i32>) {
    for x in v {
        println!("{}", x);
    }
}

struct Matrix<T> {
    data: Vec<Vec<T>>,
}

impl<T> Matrix<T>
where
    T: Mul<Output = T> + std::iter::Sum + Copy,
{
    fn mul(self, other: Vec<T>) -> Vec<T> {
        self.data
            .iter()
            .map(|row| row.dot(&other).unwrap())
            .collect::<Vec<T>>()
    }
}

trait DotProduct<T> {
    fn dot(&self, other: &Vec<T>) -> Option<T>;
}

impl<T> DotProduct<T> for Vec<T>
where
    T: Mul<Output = T> + std::iter::Sum + Copy,
{
    fn dot(&self, other: &Vec<T>) -> Option<T> {
        if self.len() != other.len() {
            return None;
        }

        let d = self
            .iter()
            .zip(other.iter())
            .map(|(a, b)| *a * *b)
            .sum::<T>();

        Some(d)
    }
}
