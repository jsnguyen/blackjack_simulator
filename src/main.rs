use rand::thread_rng;
use rand::Rng;
use std::fmt;

#[derive(Copy, Clone)]
enum Suit {
	Spade,
	Heart,
	Diamond,
	Club,
}

impl fmt::Display for Suit {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Suit::Spade => write!(f, "Spade"),
			Suit::Heart => write!(f, "Heart"),
			Suit::Diamond => write!(f, "Diamond"),
			Suit::Club => write!(f, "Club"),
		}
	}
}

#[derive(Copy, Clone)]
struct Card {
	number: u8,
	suit: Suit,
	color: bool, // true is black, false is red
}

impl Card {
	fn new(num: u8, sui: Suit, col: bool) -> Card {
		Card{number: num, suit: sui, color: col}
	}

	fn print(&self) {

		let color_str;
		if self.color {
			color_str = "Black";
		} else {
			color_str = "Red";
		}

		println!("--- Card ---");
		println!("Number: {}",self.number);
		println!("  Suit: {}",self.suit);
		println!(" Color: {}",color_str);

	}
}

struct Deck {
	cards: [Card ; 52],
}

impl Deck {

	fn new() -> Deck {
		//let mut cards : [Card ; 52] = [Card{number: 0, suit: Suit::Spade, color: false}; 52];
		let mut cards : [Card ; 52] = [Card::new(0, Suit::Spade, false); 52];

		let mut counter = 0;
		for i in 0..4 {
			let sui: Suit;

			match i {
				0 => sui = Suit::Spade,
				1 => sui = Suit::Heart,
				2 => sui = Suit::Diamond,
				_ => sui = Suit::Club,
			}

			let col: bool;
			match sui {
				Suit::Spade => col = true,
				Suit::Heart => col = false,
				Suit::Diamond => col = false,
				Suit::Club => col = true,
			}
			 
			for j in 2..=14 {
				cards[counter].number = j as u8; 
				cards[counter].suit = sui;
				cards[counter].color = col;
				counter+=1;
			}
		}

		Deck{cards: cards}
	}

    fn swap(&mut self, i: usize, j: usize) {
        let temp : Card;
        temp = self.cards[i];
        self.cards[i] = self.cards[j];
        self.cards[j] = temp;
    }

	// shuffle using Fisher-Yates
	fn shuffle(&mut self) {
        let mut new_deck : Deck = Deck::new();

        let mut rng = thread_rng();
        let mut roll;

        // last number doesn't get swapped anywhere
        for i in (1..52).rev() {
            roll = rng.gen_range(0..i);
            self.swap(i,roll);
        }
	}

}


fn main() {
	let mut deck: Deck = Deck::new();

	for card in deck.cards.iter() {
		card.print();
	}


    deck.shuffle()

}
