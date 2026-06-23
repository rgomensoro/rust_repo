use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug)]

struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];
        let values = ["2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King", "Ace"];

        let mut cards = Vec::new();
        for suit in suits {
            for value in values {
                cards.push(format!("{} of {}", value, suit));
            }
        }

        Deck { cards }
    }

    fn shuffle(&mut self) {

        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal (&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
} 


fn main() {

    let mut deck = Deck::new();

    println!("Deck: {:#?}", deck);

    deck.shuffle();

    println!("Shuffled Deck: {:#?}", deck);

    let mut hand = deck.deal(5);

    println!("Hand One: {:#?}", hand);

    hand = deck.deal(5);

    println!("Hand Two: {:#?}", hand);


}
