use strum_macros::EnumIter;

#[derive(PartialEq, EnumIter, Copy, Clone, Debug)]
pub enum Suit {
    Clubs = 1,
    Spades = 2,
    Hearts = 3,
    Diamonds = 4,
    Joker = 5,
}

impl Suit {
    pub fn from_number(i: u32) -> Option<Suit> {
        match i {
            1 => Some(Suit::Clubs),
            2 => Some(Suit::Spades),
            3 => Some(Suit::Hearts),
            4 => Some(Suit::Diamonds),
            5 => Some(Suit::Joker),
            _ => None,
        }
    }
}

#[derive(PartialEq, EnumIter, Copy, Clone, Debug)]
pub enum Rank {
    Ace = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Joker = 14,
}

impl Rank {
    pub fn from_number(i: u32) -> Option<Rank> {
        match i {
            1  => Some(Rank::Ace),
            2  => Some(Rank::Two),
            3  => Some(Rank::Three),
            4  => Some(Rank::Four),
            5  => Some(Rank::Five),
            6  => Some(Rank::Six),
            7  => Some(Rank::Seven),
            8  => Some(Rank::Eight),
            9  => Some(Rank::Nine),
            10 => Some(Rank::Ten),
            11 => Some(Rank::Jack),
            12 => Some(Rank::Queen),
            13 => Some(Rank::King),
            14 => Some(Rank::Joker),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    const CLUBS: char = '♠';
    const SPADES: char = '♣';
    const HEARTS: char = '♥';
    const DIAMONDS: char = '♦';

    pub fn new(suit: Suit, rank: Rank) -> Card {
        if (suit == Suit::Joker && rank != Rank::Joker) || (rank == Rank::Joker && suit != Suit::Joker) {
            panic!("Invalid card");
        }
        return Card {suit, rank};
    }

    pub fn print(&self) {
        if self.rank == Rank::Joker && self.suit == Suit::Joker {
            println!("Joker");
        } else {
            println!("{:?} of {:?}", self.rank, self.suit);
        }
    }

    fn suit_string(suit: Suit) -> char {
        match suit {
            Suit::Clubs    => return Self::CLUBS,
            Suit::Spades   => return Self::SPADES,
            Suit::Hearts   => return Self::HEARTS,
            Suit::Diamonds => return Self::DIAMONDS,
            Suit::Joker    => return ' '
        };
    }

    fn rank_string(rank: Rank) -> &'static str {
        match rank {
            Rank::Ace   => return "01",
            Rank::Two   => return "02",
            Rank::Three => return "03",
            Rank::Four  => return "04",
            Rank::Five  => return "05",
            Rank::Six   => return "06",
            Rank::Seven => return "07",
            Rank::Eight => return "08",
            Rank::Nine  => return "09",
            Rank::Ten   => return "10",
            Rank::Jack  => return "Jk",
            Rank::Queen => return "Qu",
            Rank::King  => return "Ki",
            Rank::Joker => return "Jo",
        };
    }

    pub fn pretty_print(&self) {
        let suit = Self::suit_string(self.suit);
        let rank = Self::rank_string(self.rank);
        println!("┌──────────┐");
        println!("│{}       {} │", suit, suit);
        println!("│          │");
        println!("│    {}    │", rank);
        println!("│          │");
        println!("│{}       {} │", suit, suit);
        println!("└──────────┘");
    }
}
