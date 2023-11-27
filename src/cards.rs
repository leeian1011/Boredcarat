use rand::{
    prelude::{Rng, ThreadRng},
    thread_rng,
};

#[derive(Clone, Copy)]
pub struct Card {
    pub value: Value,
    pub suit: Suit,
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.value, self.suit)
    }
}

impl Card {
    fn new(value: Value, suit: Suit) -> Self {
        Self { value, suit }
    }

    pub fn placeholder() -> Self {
        Self {
            value: Value::Ten,
            suit: Suit::Spades,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Suit {
    Spades,
    Diamonds,
    Clovers,
    Hearts,
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Spades => write!(f, "♠️"),
            Self::Diamonds => write!(f, "♦️"),
            Self::Clovers => write!(f, "♣️"),
            Self::Hearts => write!(f, "♥️"),
        }
    }
}

impl Suit {
    fn iterator() -> impl Iterator<Item = Suit> {
        [Suit::Spades, Suit::Diamonds, Suit::Clovers, Suit::Hearts].into_iter()
    }
}

#[derive(Clone, Copy)]
pub enum Value {
    Ace,
    Deuce,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    King,
    Queen,
    Jack,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Ace => write!(f, "A"),
            Value::Deuce => write!(f, "2"),
            Value::Three => write!(f, "3"),
            Value::Four => write!(f, "4"),
            Value::Five => write!(f, "5"),
            Value::Six => write!(f, "6"),
            Value::Seven => write!(f, "7"),
            Value::Eight => write!(f, "8"),
            Value::Nine => write!(f, "9"),
            Value::Ten => write!(f, "10"),
            Value::Jack => write!(f, "J"),
            Value::Queen => write!(f, "Q"),
            Value::King => write!(f, "K"),
        }
    }
}

impl From<Value> for i32 {
    fn from(value: Value) -> Self {
        match value {
            Value::Ace => 1,
            Value::Deuce => 2,
            Value::Three => 3,
            Value::Four => 4,
            Value::Five => 5,
            Value::Six => 6,
            Value::Seven => 7,
            Value::Eight => 8,
            Value::Nine => 9,
            Value::Ten => 10,
            Value::Jack => 10,
            Value::Queen => 10,
            Value::King => 10,
        }
    }
}

impl Value {
    fn iterator() -> impl Iterator<Item = Value> {
        [
            Value::Ace,
            Value::Deuce,
            Value::Three,
            Value::Four,
            Value::Five,
            Value::Six,
            Value::Seven,
            Value::Eight,
            Value::Nine,
            Value::Ten,
            Value::Jack,
            Value::Queen,
            Value::King,
        ]
        .into_iter()
    }
}

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        Self {
            cards: {
                let mut cards: Vec<Card> = vec![];

                for suit in Suit::iterator() {
                    for value in Value::iterator() {
                        cards.push(Card::new(value, suit))
                    }
                }

                cards
            },
        }
    }

    pub fn shuffle(&mut self) -> bool {
        let mut i = self.cards.len() - 1;
        let mut rng = thread_rng();
        while i != 1 {
            let rand = rng.gen_range(0..=i);

            self.cards.swap(i, rand);
            i -= 1;
        }
        true
    }

    pub fn draw(&mut self) -> Card {
        self.cards
            .pop()
            .expect("No shot this ever gets called when cards.len() == 0")
    }
}

impl std::fmt::Display for Deck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for card in &self.cards {
            match write!(f, "{}{} ", card.value, card.suit) {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}
