use std::fmt;
use strum_macros::EnumIter;

#[derive(EnumIter, Clone, PartialEq, Debug)]
pub enum CardType {
    Club,
    Diamond,
    Heart,
    Spade,
}

impl fmt::Display for CardType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CardType::Club    => write!(f, "Club"),
            CardType::Diamond => write!(f, "Diamond"),
            CardType::Heart   => write!(f, "Heart"),
            CardType::Spade   => write!(f, "Spade"),
        }
    }
}

#[derive(EnumIter, Clone, PartialEq, Debug)]
pub enum CardValue {
    A,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    J,
    Q,
    K,
}

impl fmt::Display for CardValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CardValue::A     => write!(f, "A"),
            CardValue::Two   => write!(f, "2"),
            CardValue::Three => write!(f, "3"),
            CardValue::Four  => write!(f, "4"),
            CardValue::Five  => write!(f, "5"),
            CardValue::Six   => write!(f, "6"),
            CardValue::Seven => write!(f, "7"),
            CardValue::Eight => write!(f, "8"),
            CardValue::Nine  => write!(f, "9"),
            CardValue::Ten   => write!(f, "10"),
            CardValue::J     => write!(f, "J"),
            CardValue::Q     => write!(f, "Q"),
            CardValue::K     => write!(f, "K"),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Card {
    card_type: CardType,
    card_value: CardValue,
}


impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(type: {}, value: {})", self.card_type, self.card_value)
    }
}

impl Card {
    pub fn new(card_type: CardType, card_value: CardValue) -> Card{
        Card {
            card_type,
            card_value,
        }
    }
}