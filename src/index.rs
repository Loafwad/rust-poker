use actix_web::{Responder, get};
use maud::html;

use crate::game;

#[get("/")]
pub async fn xindex() -> impl Responder {
    html! {
        html {
            head {
                title { "Poker" }
                script src="https://unpkg.com/htmx.org@2.0.4" {}
            }
            body {
                h1 { "Welcome to Poker!" }
                p { "This is a simple poker game." }

                button
                hx-get="/hand"
                hx-target="#hand"
                hx-swap="innerHTML" {
                    "Deal Hand"
                }

                div id="hand" {
                    // This is where the dealt hand will be displayed
                    p { "Your hand will appear here." }
                }
            }
        }
    }
}

#[get("/hand")]
pub async fn xdeal_hand() -> impl Responder {
    let hand = game::generate_hand();

    html! {
        html {
            p { "You have: " }
            ul {
                @for card in hand.cards.iter() {
                    li { (card) }
                }
            }
            p { "Hand Type: " (hand.evaluate()) }
        }
    }
}
