fn main() {
    loop {
        let (m, f, r) = read_score().unwrap();
        match (m.score, f.score, r.score) {
            (-1, -1, -1) => break,
            _ => (),
        }
        let grade = calc_grade(m, f, r);
        println!("{}", grade);
    }
}
struct MidtermScore {
    score: i32,
}

struct FinalScore {
    score: i32,
}

struct ReExamScore {
    score: i32,
}

enum Grade {
    A,
    B,
    C,
    D,
    F,
}

impl std::fmt::Display for Grade {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            Grade::A => "A",
            Grade::B => "B",
            Grade::C => "C",
            Grade::D => "D",
            Grade::F => "F",
        };
        write!(f, "{}", s)
    }
}

fn read_score() -> Option<(MidtermScore, FinalScore, ReExamScore)> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mfr: Vec<i32> = buf
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    if mfr.len() == 3 {
        Some((
            MidtermScore { score: mfr[0] },
            FinalScore { score: mfr[1] },
            ReExamScore { score: mfr[2] },
        ))
    } else {
        None
    }
}

fn calc_grade(m: MidtermScore, f: FinalScore, r: ReExamScore) -> Grade {
    match (m.score, f.score, r.score) {
        (-1, _, _) | (_, -1, _) => Grade::F,
        (m, f, _) if m + f >= 80 => Grade::A,
        (m, f, _) if m + f >= 65 => Grade::B,
        (m, f, _) if m + f >= 50 => Grade::C,
        (m, f, r) if m + f >= 30 && r >= 50 => Grade::C,
        (m, f, _) if m + f >= 30 => Grade::D,
        (_, _, _) => Grade::F,
    }
}
