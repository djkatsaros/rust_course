/*
 * Deck of cards program
 * deck instantiator
 * shuffle function
 * deal function
 */

use rand::{thread_rng, seq::SliceRandom};

#[derive(Debug)] // [ .. ] defines attributes for the Deck struct. inside:
                 // calls the 'derive attribute', which traits to automatically impl in this
                 // struct.
                 // calls 'Debug' trait, traits are sets of functions
struct Deck {
    // list of fields (data) that this struct will wrap up
    cards: Vec<String> //Vec arrays w/ flexible size, array data struct in rust have fixed lengths
}

// create inherent implementation
// adds functions to struct. used to define methods and associated functions == class methods!
// new is an associated function, called by Deck::assocd_fcn()
// can call methods on the instance of deck. methods call &self: such as shuffle
impl Deck {
    fn new() -> Self { // tells compiler we're returning a Deck struct, Self references type
                       // mentioned in parent impl block
                           
        let suits = ["Hearts", "Spades", "Diamonds", "Clubs"]; // will this list change? If not, use
                                                               // array [ stuff ] as signal to devs
        let values = ["Ace", "Two", "Three", "Four", "Five"];      

        let mut cards = vec![]; // declare as mutable, bindings immutable by default!

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit); // joins strings
                cards.push(card);
            }       
        }       

        // instance of Deck, deck is a 'binding' not a variable
        let deck = Deck { cards }; // because field is same as name of var, same as cards : cards
                                   // declare the deck binding (variab), optional :Deck type
                                           // annotation. RHS is a struct literal with init value for
                                           // cards field -> empty vector. ! is a macro    
    return deck;
    /*equivalently write return Deck { cards }; 
     * or Deck { cards }  this is  implicit return  functionality 
     */
    }
/*
 * associated fucntions: functionality not tied to specific instnace. 'full_deck',
 * 'with_n_cards(10)', 'empty_deck()'. 
 * methods: read or change fields on a specific instance. i.e. shuffling, removing a card, checking
 * for a card.
 */
    fn shuffle(&mut self) // reference to deck needs to be mutable
                          { 
        // need random number generator, use crate rand 
        let mut rng = thread_rng(); // data assoc'd with rng will be changing
        self.cards.shuffle(&mut rng);
     }

    fn deal(&mut self, num_cards: usize) ->Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {

    let mut deck = Deck::new();
    deck.shuffle(); // requires mutable deck!
    let cards = deck.deal(5); //TODO handle error of more cards from deal then in deck
    println!("Heres your deck {:#?}", deck);
    println!("Heres your hand: {:#?}", cards); // # is a formatter for nice printing
}

/*
 * immutable:
 * let numbers = vec![]
 * // error! cant change the value
 * numbers.push(1); // error!
 *
 *  // error! cant reassign either!
 *  numbers = vec![]
 */
