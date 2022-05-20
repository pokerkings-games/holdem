#[derive(Clone, Debug, PartialEq)]
#[warn(dead_code)]
pub enum CardSuit {
    Diamond = 0,
    Spade = 1,
    Club = 2,
    Heart = 3,
}

#[derive(Clone, Debug, PartialEq)]
#[warn(dead_code)]
pub enum CardNumber {
    Ace = 14,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Card {
    pub suit: CardSuit,
    pub number: CardNumber,
}