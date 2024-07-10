use strum::IntoEnumIterator;
use rand::Rng;

use crate::card;

///stack-like structure to deal with a deck of cards
pub struct Deck {
    pub cards: Vec<card::Card>,
}


impl Deck {
    ///creates a normal deck with all 52 cards (4 types x 13 values)
    pub fn new() -> Deck {
        let mut cards: Vec<card::Card> = Vec::new();

        for card_type in card::CardType::iter() {
            for card_value in card::CardValue::iter() {
                cards.push(card::Card::new(card_type.clone(), card_value.clone()));
            }
        }
        Deck { cards }
    }

    ///creates an empty deck with no cards
    pub fn empty() -> Deck {
        let cards: Vec<card::Card> = Vec::new();
        Deck { cards }
    }

    ///will return a boolean after verifying if the deck is empty or not
    pub fn is_empty(&self) -> bool {
        if self.cards.len() == 0 { true } else { false }
    }

    ///show all cards in the deck
    pub fn show(&self) {
        if self.cards.len() == 0 {
            println!("The deck is empty.");
            return
        }

        for card in &self.cards {
            println!("{}", card);
        }
    }

    ///will shuffle the deck with Fisher–Yates shuffle algorithm
    pub fn shuffle(&mut self) {
        let size = self.cards.len();

        for i in 0..size {
            let j = rand::thread_rng().gen_range(0..size);
            self.cards.swap(i as usize, j as usize);
        }
    }

    ///Removes the last card from the deck and returns it
    pub fn remove_card(&mut self) -> Option<card::Card> {
        match self.cards.pop() {
            None => None,
            Some(card) => Some(card), 
        }
    }

    //Inserts a card at the top (end) of the deck
    pub fn insert_card(&mut self, card: card::Card) {
        self.cards.push(card)
    } 
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_test() {
        let mut deck = Deck::new();

        let last_card = deck.remove_card();
        match last_card {
            None => println!("Baralho vázio"),
            Some(card) => assert_eq!(card::Card::new(card::CardType::Spade, card::CardValue::K), card),
            }
    }

    #[test]
    fn empty_test() {
        let deck = Deck::empty();
        assert_eq!(deck.is_empty(), true);
    }
}

