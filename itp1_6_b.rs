use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt;

fn main() {
    let n = read_input().parse::<i32>().unwrap();
    let cards = read_cards(&n);
    let card_set = cards.into_iter().collect::<HashSet<Card>>();

    let all_card_set = all_cards();

    let mut remain_cards = all_card_set.difference(&card_set).collect::<Vec<&Card>>();

    remain_cards.sort();
    remain_cards.iter().for_each(|c| println!("{}", c));
}

fn read_input() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn read_cards(n: &i32) -> Vec<Card> {
    let mut cards = Vec::new();
    for _ in 0..*n {
        let s = read_input();
        cards.push(Card::from_str(&s));
    }
    cards
}

fn all_cards() -> HashSet<Card> {
    let mut cards = HashSet::new();
    for s in &[Suit::S, Suit::H, Suit::C, Suit::D] {
        for v in 1..14 {
            cards.insert(Card { suit: *s, value: v });
        }
    }

    cards
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Card {
    suit: Suit,
    value: i32,
}

impl Card {
    fn from_str(s: &str) -> Self {
        let sv = s.split_whitespace().collect::<Vec<&str>>();
        let suit = Suit::from_str(sv[0]);
        let value = sv[1].parse::<i32>().unwrap();
        Card { suit, value }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.suit, self.value)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.suit
            .cmp(&other.suit)
            .then_with(|| self.value.cmp(&other.value))
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Suit {
    S,
    H,
    C,
    D,
}

impl Suit {
    fn from_str(s: &str) -> Self {
        match s {
            "S" => Suit::S,
            "H" => Suit::H,
            "C" => Suit::C,
            "D" => Suit::D,
            _ => panic!("Invalid suit"),
        }
    }

    fn order(&self) -> i32 {
        match self {
            Suit::S => 0,
            Suit::H => 1,
            Suit::C => 2,
            Suit::D => 3,
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Suit::S => "S",
            Suit::H => "H",
            Suit::C => "C",
            Suit::D => "D",
        };
        write!(f, "{}", s)
    }
}

impl PartialOrd for Suit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Suit {
    fn cmp(&self, other: &Self) -> Ordering {
        self.order().cmp(&other.order())
    }
}
