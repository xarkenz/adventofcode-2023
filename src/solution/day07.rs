use super::*;

use std::collections::BTreeMap;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from_char_p1(ch: char) -> Self {
        match ch {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => panic!("{ch}")
        }
    }

    fn from_char_p2(ch: char) -> Self {
        match ch {
            'J' => Self::Joker,
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => panic!("{ch}")
        }
    }
}

#[derive(Clone, PartialEq, Eq, Ord, Debug)]
struct Hand(Vec<Card>);

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn from_input_p1(input: &str) -> Self {
        Self(input.chars().map(|ch| Card::from_char_p1(ch)).collect())
    }

    fn from_input_p2(input: &str) -> Self {
        Self(input.chars().map(|ch| Card::from_char_p2(ch)).collect())
    }

    fn get_hand_type(&self) -> HandType {
        let mut joker_count: u64 = 0;
        let mut occurrences: BTreeMap<Card, u64> = BTreeMap::new();
        for card in &self.0 {
            if card == &Card::Joker {
                joker_count += 1;
            }
            else if let Some(previous) = occurrences.get_mut(card) {
                *previous += 1;
            }
            else {
                occurrences.insert(*card, 1);
            }
        }
        if joker_count == 5 {
            return HandType::FiveOfAKind;
        }
        let max_of_a_kind = *occurrences.values().max().unwrap();
        if joker_count > 0 {
            let joker_target = occurrences.values_mut().find(|count| **count == max_of_a_kind).unwrap();
            *joker_target += joker_count;
        }
        let max_of_a_kind = *occurrences.values().max().unwrap();
        match occurrences.len() {
            1 => {
                HandType::FiveOfAKind
            },
            2 => {
                if max_of_a_kind == 4 {
                    HandType::FourOfAKind
                }
                else {
                    HandType::FullHouse
                }
            },
            3 => {
                if max_of_a_kind == 3 {
                    HandType::ThreeOfAKind
                }
                else {
                    HandType::TwoPair
                }
            },
            4 => {
                HandType::OnePair
            },
            5 => {
                HandType::HighCard
            },
            len => panic!("{len}")
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_type = self.get_hand_type();
        let other_type = other.get_hand_type();
        match self_type.partial_cmp(&other_type) {
            Some(std::cmp::Ordering::Equal) => self.0.partial_cmp(&other.0),
            ordering => ordering
        }
    }
}

pub fn run() {
    let lines = get_input("day07.txt").lines().map(expect_line);

    let mut hands_p1: Vec<(Hand, u64)> = Vec::new();
    let mut hands_p2: Vec<(Hand, u64)> = Vec::new();
    
    for line in lines {
        let hand_p1 = Hand::from_input_p1(&line[..5]);
        let hand_p2 = Hand::from_input_p2(&line[..5]);
        let bid = line[6..].parse().unwrap();

        hands_p1.push((hand_p1, bid));
        hands_p2.push((hand_p2, bid));
    }

    hands_p1.sort_unstable();
    hands_p2.sort_unstable();

    let total_winnings_p1: u64 = hands_p1.iter().enumerate().map(|(rank, (_, bid))| (rank + 1) as u64 * bid).sum();
    let total_winnings_p2: u64 = hands_p2.iter().enumerate().map(|(rank, (_, bid))| (rank + 1) as u64 * bid).sum();

    println!("{total_winnings_p1}");
    println!("{total_winnings_p2}");
}
