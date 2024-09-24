use rand::seq::SliceRandom;
use rand::thread_rng;

enum Suit {
    Clubs,
    Hearts,
    Spades,
    Diamonds,
}

struct Card {
    suit: Suit,
    num: i32,
    value: i32,
}
impl Card {
    fn new(suit: Suit, num: i32) -> Self {
        Card {
            suit,
            num,
            value: num,
        }
    }
    fn to_string(&self) -> String {
        let suit_string = match self.suit {
            Suit::Clubs => "Clubs",
            Suit::Hearts => "Hearts",
            Suit::Spades => "Spades",
            Suit::Diamonds => "Diamonds",
        };
        let num_string = match self.num {
            1 => "Ace".to_string(),
            11 => "Jack".to_string(),
            12 => "Queen".to_string(),
            13 => "King".to_string(),
            number => number.to_string(),
        };
        format!("{} of {}", num_string, suit_string)
    }
}

fn main() {
    let mut deck: Vec<Card> = Vec::new();
    for i in 0..4 {
        for j in 1..=13 {
            let current_suit = match i {
                0 => Suit::Clubs,
                1 => Suit::Hearts,
                2 => Suit::Spades,
                3 => Suit::Diamonds,
                _ => panic!(),
            };
            deck.push(Card::new(current_suit, j));
        }
    }
    let mut rng = thread_rng();
    deck.shuffle(&mut rng);
    for card in deck {
        println!("{}", card.to_string());
    }
}
