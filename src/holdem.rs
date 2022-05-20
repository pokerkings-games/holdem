use std::cmp::min;
use crate::card::Card;
use crate::history::{History, HistoryItem};
use crate::msgs::{Action, ExecuteMsg, GameError, Status};
use crate::player::{PlayerInitMsg};
use crate::pots::{Pots};

#[derive(Debug, PartialEq, Clone)]
pub struct Holdem {
    pub small_blind: u128,
    pub big_blind: u128,
    pub bet_amount: u128,

    pub last_raised_player: String,
    pub turn: String,
    pub status: Status,

    pub pots: Pots,

    pub community_cards:Vec<Card>,
    history: History,
}

impl Holdem {
    pub fn new(small_blind:u128, big_blind:u128, players:Vec<PlayerInitMsg>) -> Result<Holdem, GameError> {
        let small_blind_player_id = players[0].clone().id;

        let mut holdem = Holdem {
            last_raised_player: small_blind_player_id.to_string(),
            small_blind,
            big_blind,
            community_cards: vec![],
            status: Status::PreFlop,
            turn: small_blind_player_id.to_string(),
            pots: Pots::new(players.clone())?,
            bet_amount: big_blind,
            history: History {
                player_cards: players.clone().iter().map(|p| {
                    return (p.id.clone(), p.cards.clone())
                }).collect(),
                list: vec![]
            },
        };

        holdem.add_history(Action::PreFlop, &"dealer".to_string(), 0);

        return Ok(holdem)
    }

    pub fn execute(
        &mut self,
        msg: ExecuteMsg,
    ) -> Result<(), GameError> {
        match msg {
            ExecuteMsg::AddCommunityCard { cards } => self.add_community_card(cards),
            ExecuteMsg::SmallBlind {player_id} => self.small_blind(&player_id),
            ExecuteMsg::BigBlind {player_id} => self.big_blind(&player_id),
            ExecuteMsg::CheckStraddle{player_id} => self.check_straddle(&player_id),
            ExecuteMsg::SetFlop{ cards } => self.set_flop(cards),
            ExecuteMsg::SetTurn{ card } => self.set_turn(card),
            ExecuteMsg::SetRiver{ card } => self.set_river(card),
            ExecuteMsg::Check{ player_id } => self.check(&player_id),
            ExecuteMsg::Call{ player_id } => self.call(&player_id),
            ExecuteMsg::Raise{ player_id, amount } => self.raise(&player_id, amount),
            ExecuteMsg::Fold{ player_id } => self.fold(&player_id),
        }
    }

    pub fn query() {

    }

    fn add_community_card(&mut self, cards:Vec<Card>) -> Result<(), GameError> {
        if self.community_cards.len() >= 5 || cards.len() == 0 || cards.len() > 3 {
            return Err(GameError::InvalidAction);
        }

        for card in cards.iter() {
            self.community_cards.push(card.clone());
        }

        self.add_history(Action::AddCommunityCard {cards}, &"dealer".to_string(), 0);
        Ok(())
    }

    fn set_flop(&mut self, cards:Vec<Card>) -> Result<(), GameError> {
        if self.status != Status::PreFlop && self.community_cards.len() != 0{
            return Err(GameError::InvalidAction);
        }

        if cards.len() != 3 {
            return Err(GameError::InvalidAction);
        }

        self.next_status()?;
        self.add_community_card(cards)?;

        Ok(())
    }

    fn set_turn(&mut self, card:Card) -> Result<(), GameError> {
        if self.status != Status::Flop && self.community_cards.len() != 3 {
            return Err(GameError::InvalidAction);
        }
        self.next_status()?;
        self.add_community_card(vec![card])?;

        Ok(())
    }

    fn set_river(&mut self, card:Card) -> Result<(), GameError> {
        if self.status != Status::Turn && self.community_cards.len() != 4 {
            return Err(GameError::InvalidAction);
        }
        self.next_status()?;
        self.add_community_card(vec![card])?;
        Ok(())
    }

    fn small_blind(&mut self, player_id:&String) -> Result<(), GameError> {
        self.check_validation(player_id)?;

        self.bet_amount = self.small_blind;
        self.pots.add_amount(&self.turn, self.small_blind)?;
        self.add_history(Action::SmallBlind, &self.turn.clone(), self.small_blind);
        self.next_turn()?;

        Ok(())
    }

    fn big_blind(&mut self, player_id:&String) -> Result<(), GameError> {
        self.check_validation(player_id)?;

        self.bet_amount = self.big_blind;
        self.pots.add_amount(&self.turn, self.big_blind)?;
        self.last_raised_player = self.turn.clone();
        self.add_history(Action::BigBlind, &self.turn.clone(), self.big_blind);
        self.next_turn()?;

        Ok(())
    }

    pub fn check_straddle(&mut self, player_id:&String) -> Result<(), GameError> {
        self.check_validation(player_id)?;

        if self.pots.get_player(&self.turn)?.straddle {
            let amount = self.big_blind * 2;
            self.bet_amount = amount;
            self.pots.add_amount(&self.turn.clone(), amount)?;
            self.last_raised_player = self.turn.clone();
            self.add_history(Action::Straddle, &self.turn.clone(), amount);
            self.next_turn()?;
        }

        Ok(())
    }

    fn next_turn(&mut self) -> Result<(), GameError> {
        let now_player = self.pots.get_player(&self.turn)?;
        if now_player.next_player_id == now_player.id {
            self.end_game();
            return Ok(());
        }

        self.turn = now_player.next_player_id.clone();

        let closed = self.pots.is_pot_closed()?;
        if closed {
            if self.status == Status::PreFlop {
                if self.bet_amount > self.big_blind {
                    //button이 raise를 한경우에
                    self.pots.refresh_closed_pot()?;
                    // self.next_status();
                }

            } else {
                self.pots.refresh_closed_pot()?;
                // self.next_status();
            }

        }

        Ok(())
    }

    fn next_status(&mut self) -> Result<(), GameError> {
        let start_player = self.pots.get_start_player()?;
        self.last_raised_player = start_player.id.clone();
        self.turn = start_player.id.clone();
        self.bet_amount = self.big_blind;

        if self.status == Status::PreFlop {
            self.add_history(Action::Flop, &"dealer".to_string(), 0);
            self.status = Status::Flop;
        } else if self.status == Status::Flop {
            self.add_history(Action::Turn, &"dealer".to_string(), 0);
            self.status = Status::Turn;
        } else if self.status == Status::Turn {
            self.add_history(Action::River, &"dealer".to_string(), 0);
            self.status = Status::River;
        } else if self.status == Status::River {
            self.end_game();
        }

        Ok(())
    }

    fn fold(&mut self, player_id: &String) -> Result<(), GameError> {
        self.check_validation(player_id)?;
        self.check_valid_action(player_id, Action::Fold)?;

        self.pots.fold(player_id)?;

        self.add_history(Action::Fold, player_id, 0);
        self.next_turn()?;

        Ok(())
    }

    fn check(&mut self, player_id: &String) -> Result<(), GameError> {
        self.check_validation(player_id)?;
        self.check_valid_action(player_id, Action::Check)?;

        self.add_history(Action::Check, player_id, 0);

        if self.status == Status::PreFlop {
            let closed = self.pots.is_pot_closed()?;
            if closed {
                self.pots.refresh_closed_pot()?;
                // self.next_status();
            }
        } else {
            if self.pots.get_player(player_id)?.next_player_id == self.pots.get_start_player()?.id {
                //all check를 한 경우.
                // self.next_status();
            } else {
                // self.next_turn()?;
            }
        }

        Ok(())
    }

    fn call(&mut self, player_id: &String) -> Result<(), GameError> {
        self.check_validation(player_id)?;
        self.check_valid_action(player_id, Action::Call)?;

        let player_balance = self.pots.get_player_balance(player_id)?;

        let amount = self.pots.calc_add_amount(player_id, self.bet_amount)?;

        self.pots.add_amount(player_id, min(amount, player_balance))?;

        self.add_history(Action::Call, player_id, amount);
        self.next_turn()?;

        Ok(())
    }

    fn raise(&mut self, player_id: &String, raise_amount:u128) -> Result<(), GameError> {
        self.check_validation(player_id)?;
        self.check_valid_action(player_id, Action::Raise)?;

        let player_balance = self.pots.get_player_balance(player_id)?;

        if self.bet_amount > raise_amount || player_balance < raise_amount {
            return Err(GameError::InvalidAmount)
        }

        self.bet_amount = raise_amount;
        let amount = self.pots.calc_add_amount(player_id, self.bet_amount)?;
        self.pots.add_amount(player_id, amount)?;

        self.last_raised_player = player_id.to_string();

        self.add_history(Action::Raise, player_id, amount);
        self.next_turn()?;

        Ok(())
    }

    fn add_history(&mut self, action:Action, player_id:&String, amount: u128) {
        self.history.list.push(HistoryItem {
            status: self.status.clone(),
            action,
            player: player_id.clone(),
            amount: amount.clone(),
            pots: self.pots.get_summary(),
            community_cards: self.community_cards.clone(),
        })
    }

    pub fn get_history(&self) -> History {
        return self.history.clone()
    }

    pub fn get_available_action(&self, player_id:&String) -> Result<Vec<Action>, GameError> {
        let last_action = &self.history.list.last().unwrap().action;

        match self.status {
            Status::End => {
                return Ok(vec![])
            },
            Status::PreFlop => {
                if self.last_raised_player == player_id.to_string() {
                    //한바퀴 돌고 다시 시작한(마지막 raise)에게 돌아왔다.
                    return Ok(vec![Action::Fold, Action::Check, Action::Raise]);
                }
            },
            Status::Flop => {
                if last_action == &Action::Flop || last_action == &Action::Check {
                    return Ok(vec![Action::Fold, Action::Check, Action::Raise]);
                }
            },
            Status::Turn => {
                if last_action == &Action::Turn || last_action == &Action::Check {
                    return Ok(vec![Action::Fold, Action::Check, Action::Raise]);
                }
            },
            Status::River => {
                if last_action == &Action::River || last_action == &Action::Check {
                    return Ok(vec![Action::Fold, Action::Check, Action::Raise]);
                }
            },
        };

        let player_balance = self.pots.get_player_balance(player_id)?;
        if player_balance < self.bet_amount {
            Ok(vec![Action::Fold, Action::Call])
        } else {
            Ok(vec![Action::Fold, Action::Call, Action::Raise])
        }
    }

    fn end_game(&mut self) {
        self.status = Status::End
    }

    fn check_valid_action(&self, player_id:&String, action:Action) -> Result<(), GameError> {
        if !self.get_available_action(player_id)?.contains(&action) {
            return Err(GameError::CantDoAction);
        }

        Ok(())
    }

    fn check_validation(&self, player_id:&String) -> Result<(), GameError> {
        let player = self.pots.get_player(player_id)?;

        if player.next_player_id == player.id || self.status == Status::End {
            //생존자가 나 혼자임.
            return Err(GameError::GameEnd);
        }

        // if self.status == Status::River && self.pots.is_pot_closed()? && self.pots.list[0].get_total_pot_balance() > 0 {
        //     //river이고, 모든 배팅이 끝나을때.
        //     return Err(GameError::GameEnd);
        // }

        if player.fold {
            return Err(GameError::AlreadyFold)
        }

        if player_id.to_string() != self.turn {
            return Err(GameError::NotYourTurn)
        }

        //community_card length validate
        if self.status == Status::PreFlop && self.community_cards.len() != 0 {
            return Err(GameError::InvalidAction)
        } else if self.status == Status::Flop && self.community_cards.len() != 3 {
            return Err(GameError::InvalidAction)
        } else if self.status == Status::Turn && self.community_cards.len() != 4 {
            return Err(GameError::InvalidAction)
        } else if self.status == Status::River && self.community_cards.len() != 5 {
            return Err(GameError::InvalidAction)
        }


        Ok(())
    }
}