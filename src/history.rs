use std::collections::HashMap;
use crate::card::Card;
use crate::msgs::{Action, Status};

#[derive(Clone, Debug, PartialEq)]
pub struct History {
    pub player_cards: Vec<(String, Vec<Card>)>,
    pub list: Vec<HistoryItem>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HistoryItem {
    pub status: Status,
    pub action: Action,
    pub player: String,
    pub amount: u128,
    pub pots: Vec<HashMap<String, u128>>, // game pot
    pub community_cards: Vec<Card>,
}
