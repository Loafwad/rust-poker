use crate::game::{Hand, HandKind};

pub fn compare_hands(a: &Hand, b: &Hand) -> std::cmp::Ordering {
    let a_kind = a.evaluate();
    let b_kind = b.evaluate();

    match a_kind.cmp(&b_kind) {
        std::cmp::Ordering::Equal => compare_secondary(a, b, &a_kind),
        ordering => ordering,
    }
}

fn compare_secondary(hand1: &Hand, hand2: &Hand, kind: &HandKind) -> std::cmp::Ordering {
    match kind {
        HandKind::HighCard => {
            let high1 = hand1.get_highest_card();
            let high2 = hand2.get_highest_card();
            high1.cmp(&high2)
        }
        HandKind::OnePair => {
            todo!()
        }
        HandKind::TwoPair => {
            todo!()
        }
        HandKind::ThreeOfAKind => {
            todo!()
        }
        HandKind::Straight => {
            todo!()
        }
        HandKind::Flush => {
            todo!()
        }
        HandKind::FullHouse => {
            todo!()
        }
        HandKind::FourOfAKind => {
            todo!()
        }
        HandKind::StraightFlush => {
            todo!()
        }
        HandKind::RoyalFlush => std::cmp::Ordering::Equal,
    }
}
