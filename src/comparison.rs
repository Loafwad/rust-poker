use crate::game::{Hand, HandKind};

pub fn compare_kickers(kickers1: &[u32], kickers2: &[u32]) -> std::cmp::Ordering {
    for (k1, k2) in kickers1.iter().zip(kickers2.iter()) {
        match k1.cmp(k2) {
            std::cmp::Ordering::Equal => continue,
            non_eq => return non_eq,
        }
    }

    std::cmp::Ordering::Equal
}

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
            let h1 = hand1.get_highest_card();
            let h2 = hand2.get_highest_card();
            h1.cmp(&h2)
        }
        HandKind::OnePair => {
            let h1 = hand1.get_n_of_a_kind(2);
            let h2 = hand2.get_n_of_a_kind(2);

            let hand_cmp = h1.cmp(&h2);

            match hand_cmp {
                std::cmp::Ordering::Equal => {
                    let kickers1 = hand1.get_kickers_descending(2);
                    let kickers2 = hand2.get_kickers_descending(2);
                    compare_kickers(&kickers1, &kickers2)
                }
                _ => hand_cmp,
            }
        }
        HandKind::TwoPair => {
            let mut h1_pairs = hand1.get_n_of_a_kind(2);
            let mut h2_pairs = hand2.get_n_of_a_kind(2);

            // Sort to compare high and low pairs
            h1_pairs.sort_by(|a, b| b.cmp(a));
            h2_pairs.sort_by(|a, b| b.cmp(a));

            match h1_pairs[0].cmp(&h2_pairs[0]) {
                std::cmp::Ordering::Equal => match h1_pairs[1].cmp(&h2_pairs[1]) {
                    std::cmp::Ordering::Equal => {
                        let kickers1 = hand1.get_kickers_descending(4);
                        let kickers2 = hand2.get_kickers_descending(4);
                        compare_kickers(&kickers1, &kickers2)
                    }
                    cmp => cmp,
                },
                cmp => cmp,
            }
        }
        HandKind::ThreeOfAKind => {
            let h1 = hand1.get_n_of_a_kind(3);
            let h2 = hand2.get_n_of_a_kind(3);

            let hand_cmp = h1.cmp(&h2);

            match hand_cmp {
                std::cmp::Ordering::Equal => {
                    let kickers1 = hand1.get_kickers_descending(3);
                    let kickers2 = hand2.get_kickers_descending(3);
                    compare_kickers(&kickers1, &kickers2)
                }
                _ => hand_cmp,
            }
        }
        HandKind::Straight => {
            let h1 = hand1.get_highest_card();
            let h2 = hand2.get_highest_card();

            h1.cmp(&h2)
        }
        HandKind::Flush => {
            let h1 = hand1.get_highest_card();
            let h2 = hand2.get_highest_card();

            h1.cmp(&h2)
        }
        HandKind::FullHouse => {
            let h1_three = hand1.get_n_of_a_kind(3);
            let h2_three = hand2.get_n_of_a_kind(3);

            let h1_pair = hand1.get_n_of_a_kind(2);
            let h2_pair = hand2.get_n_of_a_kind(2);

            let hand_cmp = h1_three.cmp(&h2_three);

            match hand_cmp {
                std::cmp::Ordering::Equal => h1_pair.cmp(&h2_pair),
                _ => hand_cmp,
            }
        }
        HandKind::FourOfAKind => {
            let h1 = hand1.get_n_of_a_kind(4);
            let h2 = hand2.get_n_of_a_kind(4);

            let hand_cmp = h1.cmp(&h2);

            match hand_cmp {
                std::cmp::Ordering::Equal => {
                    let kickers1 = hand1.get_kickers_descending(4);
                    let kickers2 = hand2.get_kickers_descending(4);
                    compare_kickers(&kickers1, &kickers2)
                }
                _ => hand_cmp,
            }
        }
        HandKind::StraightFlush => {
            let h1 = hand1.get_highest_card();
            let h2 = hand2.get_highest_card();

            h1.cmp(&h2)
        }
        HandKind::RoyalFlush => std::cmp::Ordering::Equal,
    }
}
