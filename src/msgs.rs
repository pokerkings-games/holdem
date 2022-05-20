use crate::card::Card;

#[derive(Debug, PartialEq, Clone)]
pub enum GameResult {
    RoyalStraightFlush = 10000000000000,
    StraightFlush = 9000000000000,
    FourOfAKind = 8000000000000,
    FullHouse = 7000000000000,
    Flush = 6000000000000,
    Straight = 5000000000000,
    ThreeOfAKind = 4000000000000,
    TwoPair = 3000000000000,
    OnePair = 2000000000000,
    HighCard = 1000000000000,
}

#[derive(Debug, PartialEq, Clone)]
pub enum GameError {
    InvalidAction,
    InvalidPlayerCount,
    NotYourTurn,
    CantDoAction,
    InvalidAmount,
    PlayerNotFound,
    GameEnd,
    AlreadyFold,
    PotClosed,
    InvalidCards,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Action {
    AddCommunityCard { cards: Vec<Card>},
    //not for playing game
    SmallBlind,
    //not for playing game
    BigBlind,
    //not for playing game
    Straddle,
    //not for playing game

    PreFlop,
    Flop,
    Turn,
    River,

    Fold,
    Call,
    Check,
    Raise,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Status {
    PreFlop,
    Flop,
    Turn,
    River,
    End,
}

pub enum ExecuteMsg {
    AddCommunityCard { cards: Vec<Card> },
    SmallBlind { player_id: String },
    BigBlind { player_id: String },
    CheckStraddle { player_id: String },
    SetFlop { cards: Vec<Card>},//
    SetTurn { card: Card },
    SetRiver { card: Card },
    Check {
        player_id: String,
    },
    Call {
        player_id: String,
    },
    Raise {
        player_id: String,
        amount: u128,
    },
    Fold {
        player_id: String,
    },
}

pub enum QueryMsg {
    GetHistory {}
}