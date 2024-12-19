mod deck;

fn main() {
    let mut deck = deck::Deck::new(true);
    deck.shuffle();
    for card in deck.cards {
        card.print();
    }
}
