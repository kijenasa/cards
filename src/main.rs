mod deck;

fn main() {
    let mut deck = deck::Deck::new(true);
    deck.shuffle();

    let mut p1   = deck::Deck::empty();
    let mut p2   = deck::Deck::empty();

    deck.deal(&mut vec![&mut p1, &mut p2], 2);

    println!("p1:");
    p1.print();
    println!("p2:");
    p2.print();
}
