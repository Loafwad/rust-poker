use std::fmt::{Display, Formatter, Result};

use rand::seq::SliceRandom;
use serde::Serialize;

const HAND_SIZE: usize = 5;

#[derive(Debug, Serialize, Clone)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?} of {:?}", self.rank, self.suit)
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct Hand {
    pub cards: [Card; HAND_SIZE],
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum Suit {
    Hearts,
    Spades,
    Diamonds,
    Clubs,
}

#[derive(Debug, Serialize, Clone)]
pub enum Rank {
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

impl Rank {
    pub fn value(&self) -> u32 {
        match self {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
            Rank::Ace => 14,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

impl Display for HandType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

impl Hand {
    // The sum of the values of the cards in the hand based on their ranks
    pub fn value(&self) -> u32 {
        self.cards.iter().map(|card| card.rank.value()).sum()
    }

    pub fn rank_counts(&self) -> Vec<(u32, u32)> {
        let mut frequency: Vec<(u32, u32)> = vec![(0, 0); 15];

        for card in &self.cards {
            // Increment the count for the rank
            frequency[card.rank.value() as usize].0 += 1;
            // Set the rank value
            frequency[card.rank.value() as usize].1 = card.rank.value();
        }

        frequency.retain(|&(count, _rank)| count > 0);
        frequency.sort_by(|a, b| b.0.cmp(&a.0));

        frequency
    }

    pub fn is_flush(&self) -> bool {
        let first_suit = &self.cards[0].suit;
        self.cards.iter().all(|card| &card.suit == first_suit)
    }

    pub fn is_full_house(&self) -> bool {
        let frequency = self.rank_counts();

        let three_of_a_kind = frequency.iter().any(|(count, _)| *count == 3);
        let two_of_a_kind = frequency.iter().any(|(count, _)| *count == 2);

        three_of_a_kind && two_of_a_kind
    }

    pub fn is_straight(&self) -> bool {
        let mut values: Vec<u32> = self.cards.iter().map(|card| card.rank.value()).collect();
        values.sort_unstable();

        // Check for a straight
        for i in 0..values.len() - 1 {
            if values[i] + 1 != values[i + 1] {
                return false;
            }
        }
        true
    }

    pub fn evaluate(&self) -> HandType {
        let is_flush = self.is_flush();
        let is_straight = self.is_straight();
        let is_full_house = self.is_full_house();

        let mut ranks: Vec<u32> = self.cards.iter().map(|card| card.rank.value()).collect();
        ranks.sort_unstable();

        let counts: Vec<u32> = self.rank_counts().iter().map(|&(count, _)| count).collect();

        let mut sorted = counts.clone();
        sorted.sort_by(|a, b| b.cmp(a));

        match () {
            _ if is_flush && is_straight && ranks[0] == 10 => HandType::RoyalFlush,
            _ if is_flush && is_straight => HandType::StraightFlush,
            _ if sorted == vec![4, 1] => HandType::FourOfAKind,
            _ if is_full_house => HandType::FullHouse,
            _ if is_flush => HandType::Flush,
            _ if is_straight => HandType::Straight,
            _ if sorted == vec![3, 2] => HandType::ThreeOfAKind,
            _ if sorted == vec![2, 2, 1] => HandType::TwoPair,
            _ if sorted == vec![2, 1, 1, 1] => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

pub fn generate_hand() -> Hand {
    // Case didn't mention anything about hands being pulled from
    // the same deck, so I will assume that the deck is shuffled and
    // filled with 52 cards each time a hand is generated.

    // generate new deck
    let mut deck: Vec<Card> = Vec::new();
    for suit in [Suit::Hearts, Suit::Spades, Suit::Diamonds, Suit::Clubs] {
        for rank in [
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
            Rank::Ace,
        ] {
            deck.push(Card {
                rank,
                suit: suit.clone(),
            });
        }
    }

    let mut rng = rand::rng();
    deck.shuffle(&mut rng);

    let cards: Vec<Card> = deck.iter().take(HAND_SIZE).cloned().collect();

    Hand {
        // TODO: Can panic!
        cards: cards.try_into().unwrap_or_else(|hand: Vec<Card>| {
            panic!(
                "Expected a Vec of length {} but found {}",
                HAND_SIZE,
                hand.len()
            )
        }),
    }
}
