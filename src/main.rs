use rand::thread_rng;
use rand::Rng;
use std::fmt;
use std::io;
use std::io::Write;
use std::fs::File;


#[derive(Copy, Clone)]
enum Suit {
    Spade,
    Club,
    Heart,
    Diamond,
}

impl fmt::Display for Suit {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Suit::Spade => write!(f, "Spade"),
			Suit::Club => write!(f, "Club"),
			Suit::Heart => write!(f, "Heart"),
			Suit::Diamond => write!(f, "Diamond"),
		}
	}
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
struct Card {
	number: i32,
	suit: Suit,
	color: bool, // true is black, false is red
}

#[allow(dead_code)]
impl Card {
	fn new(num: i32, sui: Suit, col: bool) -> Card {
		Card{number: num, suit: sui, color: col}
	}

	fn print(&self) {

		let color_str;
		if self.color {
			color_str = "Black";
		} else {
			color_str = "Red";
		}

        let name: String;
        match self.number {
            11 => name = "Jack".to_owned(),
            12 => name = "Queen".to_owned(),
            13 => name = "King".to_owned(),
            14 => name = "Ace".to_owned(),
            _  => name = self.number.to_string()
        }

		println!("--- Card ---");
		println!("Number: {}",self.number);
		println!("  Name: {}",name);
		println!("  Suit: {}",self.suit);
		println!(" Color: {}",color_str);

	}
}

struct Deck {
	cards: Vec<Card>
}

impl Deck {

	fn new() -> Deck {
		let mut cards: Vec<Card> = Vec::new();

		for i in 0..4 {
			let sui: Suit;

			match i {
				0 => sui = Suit::Spade,
				1 => sui = Suit::Club,
				2 => sui = Suit::Heart,
				3 => sui = Suit::Diamond,
                _ => panic!("ERROR: Suit index is less than zero or greater than three!"),
			}

			let col: bool;
			match sui {
				Suit::Spade => col = true,
				Suit::Heart => col = false,
				Suit::Diamond => col = false,
				Suit::Club => col = true,
			}
			 
			for j in 2..=14 {
                cards.push(Card::new(j, sui, col));
			}
		}

		Deck{cards: cards}
	}

    fn swap(&mut self, i: usize, j: usize) {
        self.cards.swap(i,j);
    }

	// shuffle using Fisher-Yates
	fn shuffle(&mut self) {
        let mut rng = thread_rng();
        for i in 0..self.cards.len()-1 {
            let roll = rng.gen_range(i..self.cards.len());
            self.swap(i,roll);
        }
	}

    fn deal(&mut self, player: &mut Player, n_cards: i32) {
        for _ in 0..n_cards {
            let top_card = self.cards.pop().unwrap();
            player.hand.push(top_card);
        }
    }

}

struct Player {
	hand: Vec<Card>,
}

impl Player {
	fn new() -> Player {
        Player{hand: Vec::new()}
	}
}

fn generate_ace_point_row(n_aces: &i32) -> Vec<i32> {
    let mut ace_row: Vec<i32> =  Vec::new();
    for j in 0..=*n_aces {
        ace_row.push(*n_aces - j + 11 * j);
    }
    ace_row
}

fn calc_points(hand: &Vec<Card>) -> Vec<i32> {
    let mut possible_points: Vec<i32> = Vec::new();
    possible_points.push(0);

    let mut n_aces: i32 = 0;
    for card in hand.iter(){
        if card.number == 14 {
            n_aces+=1;
        }
    }

    if n_aces != 0 {
        possible_points = generate_ace_point_row(&n_aces);
    }

    for card in hand.iter(){
        let points: i32;

        match card.number {
            10..=13 => points = 10,
            14 => points = 0,
            _ => points = card.number,
        } 

        for pps in possible_points.iter_mut(){
            *pps += points;
        }
    }

    possible_points

}

// Player interactive blackjack that plays out on the commandline
// Just run this function to play!
#[allow(dead_code)]
fn blackjack() -> i32 {

	let mut deck: Deck = Deck::new();
    let mut house: Player = Player::new();
    let mut player: Player = Player::new();

    deck.shuffle();

    deck.deal(&mut house, 2);
    deck.deal(&mut player, 2);

    println!("*** House ***");
    for card in house.hand.iter(){
        card.print();
    }
    let mut house_points: Vec<i32> = calc_points(&house.hand);
    println!("Points -> {:?}",house_points);
    println!();

    // Player turn
    loop {
        println!("*** Player ***");
        for card in player.hand.iter(){
            card.print();
        }
        let player_points: Vec<i32> = calc_points(&player.hand);
        println!("Player points -> {:?}",player_points);

        if player_points.iter().any(|&el| el == 21) {
            println!("Player won!");
            return 2;
        }

        if player_points.iter().all(|&el| el > 21) {
            println!("Player lost!");
            return 1;
        }

        print!("[1] Hit [2] Stand: ");
        io::stdout().flush().expect("stdout Flush failed...");
        let mut p_opt: String = String::new();
        io::stdin().read_line(&mut p_opt).expect("Failed to get console input...");
        let p_opt: i32 = p_opt.trim().parse().expect("Failed to parse int...");

        match p_opt {
            1 => {
                     println!("You chose to hit!");
                     deck.deal(&mut player, 1);
                 },

            2 => {
                     println!("You chose to stand!");
                     break;
                 },
            _ => panic!("Invalid player option!")
        }
        println!("");
    }

    // House turn
    while house_points.iter().any(|&el| el <= 17) { 
        deck.deal(&mut house, 1);

        println!("*** House ***");
        for card in house.hand.iter(){
            card.print();
        }
        house_points = calc_points(&house.hand);
        println!("House points {:?}", house_points);

        if house_points.iter().all(|&el| el > 21) {
            println!("Player won!");
            return 2;
        }
    }

    // Calculating points at the end if no default winner

    let player_points: Vec<i32> = calc_points(&player.hand);
    let house_points: Vec<i32> = calc_points(&house.hand);

    let best_player_point: i32 = *player_points.iter().filter(|&&el| el <= 21).max().unwrap();
    let best_house_point: i32 = *house_points.iter().filter(|&&el| el <= 21).max().unwrap();

    println!("Player, House: {:?} {:?}", player_points, house_points);
    if best_player_point > best_house_point {
        println!("Player won!");
        return 2;
    } 
    else if best_player_point < best_house_point {
        println!("House won!");
        return 1;
    }
    else {
        println!("Draw!");
        return 0;
    }
}

// Conditional blackjack for running simulations on
// Using minimum hit condition (min_hit), if all possible points less than this value then hit, otherwise stand
// return 0 -> draw
// return 1 -> house win
// return 2 -> player win
fn conditional_blackjack(min_hit: i32) -> u8{

	let mut deck: Deck = Deck::new();
    let mut house: Player = Player::new();
    let mut player: Player = Player::new();

    deck.shuffle();

    deck.deal(&mut house, 2);
    deck.deal(&mut player, 2);

    let mut house_points: Vec<i32> = calc_points(&house.hand);

    if house_points.iter().all(|&el| el > 21) {
        return 2;
    }

    // Player turn
    loop {
        let player_points: Vec<i32> = calc_points(&player.hand);

        if player_points.iter().any(|&el| el == 21) {
            return 2;
        }

        if player_points.iter().all(|&el| el > 21) {
            return 1;
        }

        if player_points.iter().min().unwrap() < &min_hit {
            deck.deal(&mut player, 1);
        } else {
            break;
        }
    }

    // House turn
    // House must stand as soon as any possible point is greater than 17
    while house_points.iter().all(|&el| el <= 17) { 
        deck.deal(&mut house, 1);

        house_points = calc_points(&house.hand);

        if house_points.iter().all(|&el| el > 21) {
            return 2;
        }
    }

    // Calculating points at the end if no default winner

    let player_points: Vec<i32> = calc_points(&player.hand);
    let house_points: Vec<i32> = calc_points(&house.hand);

    let best_player_point: i32 = *player_points.iter().filter(|&&el| el <= 21).max().unwrap();
    let best_house_point: i32 = *house_points.iter().filter(|&&el| el <= 21).max().unwrap();

    if best_player_point > best_house_point {
        return 2;
    } 
    else if best_player_point < best_house_point {
        return 1;
    }
    else {
        return 0;
    }
}

// Run the simulation with a min_hit value ranging from 10 to 20 inclusive
// Also keeps track of hypothetical money with a 5 dollar bet
// Writes results to "results.txt" in human readable format
// 1e5 iterations seems to be the sweet spot for speed and reducing statistical errors
fn simulation() {

    let mut file = File::create("results.txt").unwrap();
    writeln!(&mut file, "min_hit draw_percent house_percent player_percent").unwrap();

    for min_hit in 10..=20 {

        let n_iter = 100000;
        let mut money = 10000;
        let bet = 5;

        println!("Min Hit: {}", min_hit);
        let mut results: Vec<u8> = Vec::new();
        for _ in 0..n_iter {
            let res = conditional_blackjack(15);
            results.push(res);

            match res {
                0 => (),
                1 => money = money - bet,
                2 => money = money + bet,
                _ => println!("Error!"),
            }

        }

        println!("Money        {:.3}", money);

        let draw_percent = results.iter().filter(|&el| *el == 0).count() as f32 / n_iter as f32 * 100.0;
        let house_percent = results.iter().filter(|&el| *el == 1).count() as f32 / n_iter as f32 * 100.0;
        let player_percent = results.iter().filter(|&el| *el == 2).count() as f32 / n_iter as f32 * 100.0;

        println!("Draw 0:     {:.3}", draw_percent);
        println!("House Win:  {:.3}", house_percent);
        println!("Player Win: {:.3}", player_percent);
        println!();

        writeln!(&mut file, "| {} | {:.3}% | {:.3}% | {:.3}% |", min_hit, draw_percent, house_percent, player_percent).unwrap();

    }

}

fn main() {
    //blackjack();
    simulation();
}
