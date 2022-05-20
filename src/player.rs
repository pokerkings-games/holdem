use crate::card::Card;

#[derive(Debug, PartialEq, Clone)]
pub struct PlayerInitMsg {
    pub id: String,
    pub balance: u128,
    pub straddle: bool,
    pub cards: Vec<Card>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Player {
    pub id: String,
    pub cards: Vec<Card>,
    pub fold: bool,
    pub straddle: bool,
    pub buy_in_amount: u128,
    pub balance: u128,
    pub prev_player_id: String,
    pub next_player_id: String,
}

impl Player {
    pub fn new(id: String, balance:u128, straddle: bool) -> Player{
        return Player {
            id,
            cards: vec![],
            fold: false,
            balance,
            buy_in_amount: balance,
            straddle,
            prev_player_id: String::new(),
            next_player_id: String::new(),
        }
    }


}