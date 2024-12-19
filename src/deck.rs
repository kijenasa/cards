use strum::IntoEnumIterator;
use rand::seq::SliceRandom;
use card::Card;
use card::Suit;
use card::Rank;

mod card;

pub struct Deck {
    pub cards: Vec<card::Card>,
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

    pub fn print(&self) {
        for card in &self.cards {
            card.print();
        }
    }

    pub fn shuffle(&mut self) {
        let mut rand = rand::thread_rng();
        self.cards.shuffle(&mut rand);
    }

    
}
