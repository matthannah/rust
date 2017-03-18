use Suit::*;
use Pip::*;

#[derive(Debug, Copy, Clone)]
enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades
}

#[derive(Debug, Copy, Clone)]
enum Pip {
    One, Two, Three, Four, Five, Six, Seven,
    Eight, Nine, Ten, Jack, Queen, King, Ace
}

#[derive(Debug)]
struct Card {
    pip: Pip,
    suit: Suit
}

#[derive(Debug)]
struct Deck( Vec<Card> );

impl Deck {
    fn new() -> Deck {
        let mut cards: Vec<Card> = Vec::new();

        for suit in &[Hearts, Diamonds, Clubs, Spades] {
            for pip in &[One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace] {
                cards.push(Card{ suit: *suit, pip: *pip})
            }
        }

        Deck(cards)
    }
}

fn main() {
    let deck = Deck::new();
    println!("{:?}", deck);
}
