use std::default;

use rand::{rng, seq::IteratorRandom, Rng};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, Clone, Copy)]
enum Suits {
    heart,
    diamond,
    spades,
    clubs
}

struct Card {
    suit: Suits,
    number:i8,
}

fn generateCard() -> Card {
    let newSuit = Suits::iter().choose(&mut rng()).unwrap();
    let newNum = rng().random_range(1..=10);

    Card { suit: newSuit, number: newNum }
}

fn suitToString(suit: Suits) -> String {
    let mut suitSting = String::new();
    match suit {
        Suits::heart => suitSting = String::from("Heart"),
        Suits::diamond => suitSting = String::from("Diamond"),
        Suits::spades => suitSting = String::from("Spades"),
        Suits::clubs => suitSting = String::from("Clubs"),
    }

    suitSting


}

impl Card {
    fn announceCard(&self) {
        println!("Suit is {} and Number is {}", suitToString(self.suit), self.number)
    } 
}

fn main() {
    let card1 = generateCard();
    card1.announceCard();   
}

