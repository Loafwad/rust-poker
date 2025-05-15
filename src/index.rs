use actix_web::{
    Error,
    error::ErrorInternalServerError,
    get, post,
    web::{self, Form},
};
use maud::{Markup, html};
use serde::Deserialize;
use serde_json::json;
use std::result::Result;

use crate::{
    comparison,
    game::{self, Card, Hand, HandKind},
};

#[get("/")]
pub async fn index() -> Result<Markup, Error> {
    xindex().await
}

pub async fn xindex() -> Result<Markup, Error> {
    Ok(html! {
        html {
            head {
                title { "Poker" }
                script src="https://unpkg.com/htmx.org@2.0.4" {}
            }
            body {
                h1 { "Welcome to Poker!" }
                p { "This is a simple poker game." }

                div {
                    button
                    hx-post="/deal"
                    hx-target="#hand"
                    hx-swap="innerHTML" {
                        "Deal Hand"
                    }

                    div id="hand" {
                        // This is where the dealt hand will be displayed
                        p { "Your hand will appear here." }
                    }
                }

                div {
                    button
                    hx-get="/history"
                    hx-target="#history"
                    hx-swap="innerHTML" {
                        "View History"
                    }

                    div id="history" {
                        // This is where the history will be displayed
                        p { "History will appear here." }
                    }
                }

                div {
                    button
                    hx-post="/compare"
                    hx-target="#compare"
                    hx-vals=(json!({
                        // Mock data for demo
                        "hand1": "[\"2h\", \"3d\", \"4s\", \"5c\", \"6h\"]",
                        "hand2": "[\"7h\", \"8d\", \"9s\", \"Tc\", \"Jh\"]"
                    }))
                    hx-swap="innerHTML" {
                        "Compare Hands"
                    }

                    div id="compare" {
                        // This is where the comparison will be displayed
                        p { "Comparison will appear here." }
                    }
                }
            }
        }
    })
}

#[post("/deal")]
pub async fn deal(pool: web::Data<sqlx::PgPool>) -> Result<Markup, Error> {
    let hand: Hand = game::generate_hand();
    let hand_kind: HandKind = hand.evaluate();
    let annotation = hand.to_string();

    sqlx::query!(
        r#"
        INSERT INTO hands (cards, hand_kind)
        VALUES ($1, $2)
        "#,
        annotation,
        hand_kind as HandKind
    )
    .execute(pool.get_ref())
    .await
    .map_err(ErrorInternalServerError)?;

    let hand = xhand(&hand);

    Ok(hand)
}

pub fn xhand(hand: &Hand) -> Markup {
    html! {
        p { "You have: " }
        ul {
            @for card in hand.cards.iter() {
                li { (card.to_label()) }
            }
        }
        p { "Hand Type: " (hand.evaluate()) }
    }
}

struct HistoryView {
    cards: String,
    hand_kind: HandKind,
}

#[get("/history")]
pub async fn history(pool: web::Data<sqlx::PgPool>) -> Result<Markup, Error> {
    let hands = sqlx::query_as!(
        HistoryView,
        r#"
        SELECT cards, hand_kind AS "hand_kind: HandKind"
        FROM hands
        "#
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(ErrorInternalServerError)?;

    let markup = html! {
        h2 { "History" }
        ul {
            @for hand in hands.iter() {
                li {
                    p { (hand.hand_kind) span { " cards: " (hand.cards) } }
                }
            }
        }
    };

    Ok(markup)
}

#[derive(Deserialize)]
struct CompareReq {
    hand1: String,
    hand2: String,
}

#[post("/compare")]
pub async fn compare_hands(form: Form<CompareReq>) -> Result<Markup, Error> {
    // Note: Very lazy approach to parsing the request body for demo purposes
    let hand1_parsed: Vec<String> = serde_json::from_str(&form.hand1).unwrap();
    let hand2_parsed: Vec<String> = serde_json::from_str(&form.hand2).unwrap();

    let hand1 = parse_hands(hand1_parsed)?;
    let hand2 = parse_hands(hand2_parsed)?;

    let winner = match comparison::compare_hands(&hand1, &hand2) {
        std::cmp::Ordering::Less => "Hand 2 wins!",
        std::cmp::Ordering::Greater => "Hand 1 wins!",
        std::cmp::Ordering::Equal => "It's a tie!",
    };

    let hand1 = xhand(&hand1);
    let hand2 = xhand(&hand2);

    Ok(html! {
        p { (winner) }
        div {
            h3 { "Hand 1" }
            (hand1)
        }
        div {
            h3 { "Hand 2" }
            (hand2)
        }
    })
}

fn parse_hands(hands: Vec<String>) -> Result<Hand, Error> {
    let mut cards: Vec<Card> = Vec::new();
    for card_str in hands {
        let card = Card::from_string(&card_str)?;
        cards.push(card);
    }
    if cards.len() != game::HAND_SIZE {
        return Err(ErrorInternalServerError("Invalid hand size"));
    }
    Ok(Hand {
        cards: cards.try_into().unwrap(),
    })
}
