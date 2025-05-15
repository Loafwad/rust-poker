use core::fmt;
use std::fmt::{Display, Formatter};

use actix_web::{Error, error::ErrorInternalServerError};
use rand::seq::SliceRandom;
use serde::Serialize;
use sqlx::Type;

pub const HAND_SIZE: usize = 5;

#[derive(Debug, Serialize, Clone)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}{:?}", self.rank.char(), self.suit.char())
    }
}

impl Card {
    /**
     * E.g "Nine of Hearts", "Ace of Spades", etc.
     */
    pub fn to_label(&self) -> String {
        format!("{:?} of {:?}", self.rank, self.suit)
    }

    pub fn from_string(s: &str) -> Result<Card, Error> {
        if s.len() != 2 {
            return Err(ErrorInternalServerError("Invalid card string length"));
        }

        let rank = match s.chars().nth(0).unwrap() {
            '2' => Rank::Two,
            '3' => Rank::Three,
            '4' => Rank::Four,
            '5' => Rank::Five,
            '6' => Rank::Six,
            '7' => Rank::Seven,
            '8' => Rank::Eight,
            '9' => Rank::Nine,
            'T' => Rank::Ten,
            'J' => Rank::Jack,
            'Q' => Rank::Queen,
            'K' => Rank::King,
            'A' => Rank::Ace,
            _ => return Err(ErrorInternalServerError("Invalid rank character")),
        };

        let suit = match s.chars().nth(1).unwrap() {
            'h' => Suit::Hearts,
            's' => Suit::Spades,
            'd' => Suit::Diamonds,
            'c' => Suit::Clubs,
            _ => return Err(ErrorInternalServerError("Invalid suit character")),
        };

        Ok(Card { rank, suit })
    }
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum Suit {
    Hearts,
    Spades,
    Diamonds,
    Clubs,
}

impl Suit {
    pub fn char(&self) -> char {
        match self {
            Suit::Hearts => 'h',
            Suit::Spades => 's',
            Suit::Diamonds => 'd',
            Suit::Clubs => 'c',
        }
    }
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
    pub fn char(&self) -> char {
        match self {
            Rank::Two => '2',
            Rank::Three => '3',
            Rank::Four => '4',
            Rank::Five => '5',
            Rank::Six => '6',
            Rank::Seven => '7',
            Rank::Eight => '8',
            Rank::Nine => '9',
            Rank::Ten => 'T',
            Rank::Jack => 'J',
            Rank::Queen => 'Q',
            Rank::King => 'K',
            Rank::Ace => 'A',
        }
    }

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

#[derive(Debug, Serialize, Clone, Type, PartialEq, Eq, PartialOrd, Ord)]
#[sqlx(type_name = "hand_kind", rename_all = "snake_case")]
pub enum HandKind {
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

impl Display for HandKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// Represents the number of cards in hand of a given rank
pub struct RankCount {
    rank: u32,
    count: u32,
}

#[derive(Debug, Serialize, Clone)]
pub struct Hand {
    pub cards: [Card; HAND_SIZE],
}

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let cards = self
            .cards
            .iter()
            .map(|card| card.rank.char().to_string() + &card.suit.char().to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{}", cards)
    }
}

impl Hand {
    pub fn get_rank_counts(&self) -> Vec<RankCount> {
        let mut counts = std::collections::HashMap::new();

        for card in &self.cards {
            let rank_value = card.rank.value();
            *counts.entry(rank_value).or_insert(0) += 1;
        }

        let mut counts: Vec<RankCount> = counts
            .into_iter()
            .map(|(rank, count)| RankCount { rank, count })
            .collect();

        counts.sort_by(|a, b| b.count.cmp(&a.count));
        counts
    }

    /**
     * Returns the ranks of the cards that have exactly n occurrences in the hand.
     */
    pub fn get_n_of_a_kind(&self, n: u32) -> Vec<u32> {
        self.get_rank_counts()
            .iter()
            .filter(|&rank_count| rank_count.count == n)
            .map(|rank_count| rank_count.rank)
            .collect()
    }

    pub fn get_kickers_descending(&self, n: u32) -> Vec<u32> {
        let excluded_ranks = self.get_n_of_a_kind(n);

        let mut kickers: Vec<u32> = self
            .cards
            .iter()
            .filter(|card| !excluded_ranks.contains(&card.rank.value()))
            .map(|card| card.rank.value())
            .collect();

        kickers.sort_by(|a, b| b.cmp(a));
        kickers
    }

    pub fn get_highest_card(&self) -> Option<u32> {
        self.cards.iter().map(|card| card.rank.value()).max()
    }

    pub fn is_flush(&self) -> bool {
        let first_suit = &self.cards[0].suit;
        self.cards.iter().all(|card| &card.suit == first_suit)
    }

    pub fn is_straight(&self) -> bool {
        let mut values: Vec<u32> = self.cards.iter().map(|card| card.rank.value()).collect();
        values.sort_unstable();
        values.dedup();

        // Normal straight
        if values.len() == 5 && values[4] - values[0] == 4 {
            return true;
        }

        // Wheel straight (A-2-3-4-5)
        values.contains(&14) && values == vec![2, 3, 4, 5, 14]
    }

    pub fn evaluate(&self) -> HandKind {
        let is_flush = self.is_flush();
        let is_straight = self.is_straight();

        let mut ranks: Vec<u32> = self.cards.iter().map(|card| card.rank.value()).collect();
        ranks.sort_unstable();

        let counts: Vec<u32> = self.get_rank_counts().iter().map(|rc| rc.count).collect();

        let mut sorted = counts.clone();
        sorted.sort_by(|a, b| b.cmp(a));

        match () {
            _ if is_flush && is_straight && ranks[0] == 10 => HandKind::RoyalFlush,
            _ if is_flush && is_straight => HandKind::StraightFlush,
            _ if sorted == vec![4, 1] => HandKind::FourOfAKind,
            _ if sorted == vec![3, 2] => HandKind::FullHouse,
            _ if is_flush => HandKind::Flush,
            _ if is_straight => HandKind::Straight,
            _ if sorted == vec![3, 1, 1] => HandKind::ThreeOfAKind,
            _ if sorted == vec![2, 2, 1] => HandKind::TwoPair,
            _ if sorted == vec![2, 1, 1, 1] => HandKind::OnePair,
            _ => HandKind::HighCard,
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
        // Should be safe? Because we know that we are only taking 5 cards
        cards: cards.try_into().unwrap(),
    }
}
