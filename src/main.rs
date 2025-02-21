use std::default;

use rand::{random_range, rng, seq::IteratorRandom, Rng};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, Clone, Copy, PartialEq)]
enum Suits {
    heart,
    diamond,
    spades,
    clubs
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

fn announceDeck(deck: [Card;5]) {
    for item in deck.iter() {
        item.announceCard();
    }
}

fn generateDeck() -> [Card;5] {
    [generateCard(), generateCard(), generateCard(), generateCard(), generateCard()]
}

fn shuffleCards(cardList: [Card;5]) -> [Option<Card>;5] {
    let mut dupCardList: [Option<Card>; 5] = [None;5];
    for i in 0..5 {
        let mut valid = false;
        while !(valid) {
            let position = rng().random_range(0..5);
            if dupCardList[position] == None {
                dupCardList[position] = Some(cardList[i]);
                valid = true
            }
        }
    }

    dupCardList
}

fn selectRandomCard(deck: [Card;5]) -> Card {
    let index = rng().random_range(0..5);
    deck[index]
}

fn main() {
    let card1 = generateCard();
    card1.announceCard();   
    let cardList: [Card; 5] = generateDeck();
    announceDeck(cardList);
    //println!("{:?}", shuffleCards(cardList));
    selectRandomCard(cardList).announceCard();
}

