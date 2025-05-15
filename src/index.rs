use actix_web::{Error, HttpRequest, error::ErrorInternalServerError, get, post, web};
use maud::{Markup, html};
use std::result::Result;

use crate::{
    comparison,
    game::{self, Hand, HandKind},
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

    fn to_code(card: &game::Card) -> String {
        format!("{}{}", card.rank.char(), card.suit.char())
    }

    let cards: Vec<String> = hand.cards.iter().map(to_code).collect();
    let cards: String = cards.join(", ");

    let hand_kind: HandKind = hand.evaluate();

    sqlx::query!(
        r#"
        INSERT INTO hands (cards, hand_kind)
        VALUES ($1, $2)
        "#,
        cards,
        hand_kind as HandKind
    )
    .execute(pool.get_ref())
    .await
    // TODO: Handle error properly
    .map_err(ErrorInternalServerError)?;

    let hand = xhand(&hand);

    Ok(hand)
}

pub fn xhand(hand: &Hand) -> Markup {
    html! {
        p { "You have: " }
        ul {
            @for card in hand.cards.iter() {
                li { (card) }
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
    // TODO: Handle error properly
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

#[post("/compare")]
pub async fn generate_two_and_compare(_req: HttpRequest) -> Result<Markup, Error> {
    // TODO: Parse hands from request

    let hand1 = game::generate_hand();
    let hand2 = game::generate_hand();

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
