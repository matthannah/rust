extern crate rand;

use rand::Rng;
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

    fn shuffle(&mut self) {
        rand::thread_rng().shuffle(self.0.as_mut_slice());
    }
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Hearts   => write!(f, "H"),
            Diamonds => write!(f, "D"),
            Clubs    => write!(f, "C"),
            Spades   => write!(f, "S"),
        }
    }
}

impl std::fmt::Display for Pip {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            One   => write!(f, "1"),
            Two   => write!(f, "2"),
            Three => write!(f, "3"),
            Four  => write!(f, "4"),
            Five  => write!(f, "5"),
            Six   => write!(f, "6"),
            Seven => write!(f, "7"),
            Eight => write!(f, "8"),
            Nine  => write!(f, "9"),
            Ten   => write!(f, "10"),
            Jack  => write!(f, "J"),
            Queen => write!(f, "Q"),
            King  => write!(f, "K"),
            Ace   => write!(f, "A")
        }
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}{}]", self.pip, self.suit)
    }
}

impl std::fmt::Display for Deck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (count, card) in self.0.iter().enumerate() {
            if count % 4 == 0 { try!(write!(f, "\n")); }
            try!(write!(f, "{}", card));
        }

        write!(f, "")
    }
}

fn main() {
    let mut deck = Deck::new();
    println!("{}", deck);
    deck.shuffle();
    println!("{}", deck)
}
