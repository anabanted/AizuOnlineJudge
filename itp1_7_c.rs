fn main() {
    let (r, c) = read_rc();
    let table = read_table(r, c).unwrap();
    let table_with_sum = table.table_with_sum();
    println!("{}", table_with_sum);
}

fn read_rc() -> (usize, usize) {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let r = iter.next().unwrap().parse().unwrap();
    let c = iter.next().unwrap().parse().unwrap();
    (r, c)
}

fn read_table(r: usize, c: usize) -> Option<Table> {
    let mut rows = Vec::new();
    for _ in 0..r {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let data = buf
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i32>>();

        if data.len() != c {
            return None;
        }

        rows.push(Row::new(data));
    }
    Some(Table::new(rows))
}

#[derive(Clone)]
struct Row {
    data: Vec<i32>,
}

impl Row {
    fn new(data: Vec<i32>) -> Row {
        Row { data }
    }

    fn sum(&self) -> i32 {
        self.data.iter().sum()
    }

    fn row_with_sum(&self) -> Row {
        let mut data = self.data.clone();
        data.push(self.sum());
        Row::new(data)
    }
}

impl std::fmt::Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.data
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

struct Table {
    rows: Vec<Row>,
}

impl Table {
    fn new(rows: Vec<Row>) -> Table {
        Table { rows }
    }

    fn sum_i(&self, i: usize) -> i32 {
        self.rows.iter().map(|row| row.data[i]).sum()
    }

    fn sum(&self) -> i32 {
        self.rows.iter().map(|row| row.sum()).sum()
    }

    fn sum_row(&self) -> Row {
        let mut data = (0..self.rows[0].data.len())
            .map(|i| self.sum_i(i))
            .collect::<Vec<i32>>();
        data.push(self.sum());
        Row::new(data)
    }

    fn table_with_sum(&self) -> Table {
        let mut rows = self
            .rows
            .iter()
            .map(|row| row.row_with_sum())
            .collect::<Vec<Row>>();
        rows.push(self.sum_row());
        Table::new(rows)
    }
}

impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = self
            .rows
            .iter()
            .map(|row| row.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", s)
    }
}
