use strum::IntoEnumIterator;
use rand::seq::SliceRandom;
use crate::card::Card;
use crate::card::Suit;
use crate::card::Rank;

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new(jokers: bool) -> Deck {
        let mut deck = Deck {cards: Vec::new()};
        for suit in Suit::iter() {
            if suit == Suit::Joker {
                continue;
            }
            for rank in Rank::iter() {
                if rank == Rank::Joker {
                    continue;
                }
                deck.cards.push(Card::new(suit, rank));
            }
        }
        if jokers {
            deck.cards.push(Card::new(Suit::Joker, Rank::Joker));
            deck.cards.push(Card::new(Suit::Joker, Rank::Joker));
        }
        return deck;
    }

    pub fn empty() -> Deck {
        return Deck {cards: Vec::new()};
    }

    pub fn deal(&mut self, decks: &mut Vec<&mut Deck>, count: u32) {
        for _ in 1..=count {
            for deck in decks.iter_mut() {
                deck.cards.push(match self.cards.pop() {
                    Some(s) => s,
                    None    => todo!("ran out of cards"),
                });
            }
        }
    }

    pub fn shuffle(&mut self) {
        let mut rand = rand::thread_rng();
        self.cards.shuffle(&mut rand);
    }

    pub fn print(&self) {
        for card in &self.cards {
            card.print();
        }
    }
}
