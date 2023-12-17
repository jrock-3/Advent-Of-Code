use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline, one_of},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult, Parser,
};

fn main() {
    let input = include_str!("../in/in1.txt");
    dbg!(process(input));
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Card {
    A = 13,
    K = 12,
    Q = 11,
    J = 0,
    T = 9,
    Nine = 8,
    Eight = 7,
    Seven = 6,
    Six = 5,
    Five = 4,
    Four = 3,
    Three = 2,
    Two = 1,
}

impl Card {
    fn new(c: char) -> Card {
        match c {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

impl HandType {
    fn from(cards: &[Card]) -> HandType {
        let num_j = cards.iter().filter(|&&card| card == Card::J).count() as u32;

        let mut counts = HashMap::new();
        for card in cards.iter().filter(|&&card| card != Card::J) {
            let card_val = *card as u32;
            *counts.entry(card_val).or_insert(0u32) += 1;
        }

        let mut counts = counts.into_values().collect::<Vec<_>>();
        counts.sort();
        if let Some(last) = counts.last_mut() {
            *last += num_j;
        } else {
            counts.push(num_j);
        }

        match counts[..] {
            [5] => HandType::FiveOfAKind,
            [1, 4] => HandType::FourOfAKind,
            [2, 3] => HandType::FullHouse,
            [1, 1, 3] => HandType::ThreeOfAKind,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 1, 2] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    hand_type: HandType,
    cards: Vec<Card>,
    bid: u32,
}

impl Hand {
    fn new(raw: Vec<char>, bid: u32) -> Hand {
        let cards = raw.iter().map(|&c| Card::new(c)).collect::<Vec<_>>();
        let hand_type = HandType::from(&cards);
        Hand {
            cards,
            bid,
            hand_type,
        }
    }
}

fn get_hands(input: &str) -> IResult<&str, Vec<Hand>> {
    let (input, hands) = separated_list1(
        newline,
        separated_pair(many1(one_of("AKQJT98765432")), tag(" "), complete::u32)
            .map(|(chars, num)| Hand::new(chars, num)),
    )(input)?;

    Ok((input, hands))
}

fn process(input: &str) -> String {
    let (_, mut hands) = get_hands(input).unwrap();

    hands.sort();

    let res = hands
        .iter()
        .enumerate()
        .fold(0u32, |acc, (i, hand)| acc + (i as u32 + 1) * hand.bid);

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in2() {
        let input = include_str!("../in/sample1.txt");
        assert_eq!("5905", process(input));
    }
}
