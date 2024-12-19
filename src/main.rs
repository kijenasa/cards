mod deck;

fn main() {
    let mut deck = deck::Deck::new(true);
    deck.shuffle();
    deck.print();
}
