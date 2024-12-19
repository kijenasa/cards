mod deck;

fn main() {
    let deck = deck::Deck::new(true);
    for card in deck.cards {
        card.print();
    }
}
