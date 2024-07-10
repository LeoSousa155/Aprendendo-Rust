mod card;
mod deck;

fn main() {
    let mut deck = deck::Deck::new();
    deck.show();
    println!("-------------------------");
    println!("--------SHUFFLING--------");
    println!("-------------------------");
    deck.shuffle();
    deck.show();
    println!("-------------------------");
    println!("-----REMOVING-A-CARD-----");
    println!("-------------------------");
    let last_card = deck.remove_card();
    deck.show();
    match last_card {
        None => println!("Baralho vázio"),
        Some(ref card) => println!("Carta removida: {}", card),
    }
    println!("-------------------------");


    println!("testing an empty deck: ");
    let mut empty = deck::Deck::empty();
    empty.show();
    println!("----------------------------------------------");
    println!("---ADDING A CARD FROM A DECK INTO ANOTHER ----");
    println!("----------------------------------------------");
    match last_card {
        None => println!("Can`t insert none in the deck"),
        Some(card) => empty.insert_card(card)
    }
}
