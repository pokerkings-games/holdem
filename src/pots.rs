use std::collections::HashMap;
use crate::msgs::GameError;
use crate::player::{Player, PlayerInitMsg};
use crate::pot::Pot;

#[derive(Debug, PartialEq, Clone)]
pub struct Pots {
    pub players: Vec<Player>,
    pub list: Vec<Pot>,
    pub closed: Vec<Pot>,
}

impl Pots {
    pub fn new(player_init_msg: Vec<PlayerInitMsg>) -> Result<Pots, GameError> {
        if player_init_msg.len() < 2 || player_init_msg.len() > 8 {
            return Err(GameError::InvalidPlayerCount)
        }

        let mut players:Vec<Player> = vec![];

        //make linked list
        for index in 0..player_init_msg.len() {
            let p = &player_init_msg[index];

            let prev_index = (index + player_init_msg.len() - 1) % player_init_msg.len();
            let next_index = (index + 1) % player_init_msg.len();

            let player = Player {
                id: p.id.clone(),
                cards: p.cards.clone(),
                fold: false,
                balance: p.balance,
                buy_in_amount: p.balance,
                straddle: p.straddle.clone(),
                prev_player_id: player_init_msg[prev_index].id.to_string(),
                next_player_id: player_init_msg[next_index].id.to_string(),
            };

            players.push(player)
        }

        let mut pots = Pots {
            players,
            list: vec![],
            closed: vec![],
        };

        pots.create_new_pot()?;
        Ok(pots)
    }

    pub fn get_start_player(&self) -> Result<&Player, GameError> {
        let mut player = self.get_player(&self.players[0].id)?; //small blind

        loop {
            //모두가 fold되어 무한루프가 되는일은 발생하지 않는다.
            if player.fold {
                player = self.get_player(&player.next_player_id)?;
            } else {
                break;
            }
        }

        Ok(player)
    }

    pub fn get_player(&self, player_id: &String) -> Result<&Player, GameError> {
        for player in self.players.iter() {
            if player.id == player_id.to_string() {
                return Ok(player);
            }
        }

        Err(GameError::PlayerNotFound)
    }

    pub fn create_new_pot(&mut self) -> Result<(), GameError> {
        self.list.push(Pot {
            balance: HashMap::new(),
            max_amount: None,
        });

        Ok(())
    }

    pub fn get_playable_players(&self) -> Result<Vec<Player>, GameError> {
        let mut alive_players = vec![];
        for player in self.players.iter() {
            if player.balance > 0 && !player.fold {
                alive_players.push(player.clone());
            }
        }

        Ok(alive_players)
    }

    pub fn add_amount(&mut self, player_id: &String, amount: u128) -> Result<(), GameError> {
        let mut is_all_in = false;

        for p in &mut self.players {
            if p.id.to_string() == player_id.to_string() {
                if p.balance < amount {
                    return Err(GameError::InvalidAmount);
                } else {
                    p.balance -= amount;

                    if p.balance == 0 {
                        is_all_in = true;
                    }
                }
            }
        }

        let mut amount = amount;
// assert_eq!(last_pot.max_amount, None);


        for pot in &mut self.list {
            let balance = pot.get_total_bet_amount(player_id);
            if let Some(max_amount) = pot.max_amount {
                //누군가 allin해서 종료된 팟.
                //max_amount = 종료된 팟의 키높이를 맞추기위한 최대값.
                let a = max_amount - balance;
                pot.add_amount(player_id, a)?;
                amount -= a;
            } else {
                //현재 살아있는 팟.(사이드팟)
                pot.add_amount(player_id, amount)?;
                amount = 0;
            }
        }

        if amount > 0 {
            //짜투리가 남으면 새로운 팟
            self.create_new_pot()?;
            let mut created_pot = self.list.pop().unwrap();
            created_pot.balance.insert(player_id.to_string(), amount);
            self.list.push(created_pot);
        }

        if is_all_in {
            let mut last_pot = self.list.pop().unwrap();

            if last_pot.max_amount.is_none() {
                let max_amount = last_pot.get_total_bet_amount(player_id);
                last_pot.max_amount = Some(max_amount);
            }

            // if player_id.to_string() == "p0".to_string() && self.players[0].balance == 0 {
            //     assert_eq!(self.list[0].balance, vec![]);
            //     // assert_eq!(last_pot.get_total_bet_amount(player_id)?, 190);
            //     assert_eq!(last_pot.max_amount, None);
            // }

            let overflow_amount:HashMap<String, u128> = last_pot.get_overflow_amount();
            //초과분을 다음팟(사이드팟)으로 넘기기 위함

            self.list.push(last_pot);

            if overflow_amount.len() > 0 {
                self.create_new_pot()?;

                let mut created_pot = self.list.pop().unwrap();
                created_pot.balance = overflow_amount.clone();
                self.list.push(created_pot);
            }
        }

        return Ok(())
    }

    pub fn fold(&mut self, player_id:&String) -> Result<(), GameError> {
        // for p in &mut self.players {
        //     if p.id.to_string() == player.to_string() {
        //         p.fold = true;
        //     }
        // }

        // rebuild linkedlist without fold player.
        let fold_player = self.get_player(player_id)?;
        let prev_player_id = &fold_player.clone().prev_player_id;
        let next_player_id = &fold_player.clone().next_player_id;

        for player in &mut self.players {
            if player.id == prev_player_id.clone() {
                player.next_player_id = next_player_id.clone();
            } else if player.id == next_player_id.clone() {
                player.prev_player_id = prev_player_id.clone();
            } else if player.id == player_id.to_string() {
                player.fold = true;
            }
        }

        Ok(())
    }

    pub fn get_player_balance(&self, player:&String) -> Result<u128, GameError> {
        Ok(self.get_player(player)?.balance)
    }

    pub fn is_pot_closed(&self) -> Result<bool, GameError> {
        let mut closed = true;
        for pot in self.list.iter() {
            let mut alive:u8 = 0;
            for player in self.players.iter() {
                if !player.fold && player.balance != 0 {
                    alive += 1;
                }
            }

            if pot.balance.len() < alive as usize {
                closed = false;
                continue;
            }

            let mut value:u128 = 0;
            for (player_id, balance) in pot.balance.iter() {
                let player = self.get_player(player_id)?;
                if balance.clone() > 0 && value == 0 && !player.fold {
                    //fold한 사용자 무시,
                    //팟의 모든 사용자의 금액이 동일하면 이 팟은 끝난거임.
                    value = balance.clone();
                }

                if balance.clone() != value {
                    closed = false;
                }
            }
        }

        Ok(closed)
    }

    pub fn refresh_closed_pot(&mut self) -> Result<(), GameError> {
        for pot in self.list.iter() {
            self.closed.push(pot.clone());
        }

        self.list.clear();
        self.create_new_pot()?;

        Ok(())
    }

    pub fn calc_add_amount(&self, player_id:&String, bet_amount: u128) -> Result<u128, GameError> {
        let mut sum = 0;
        for pot in self.list.iter() {
            sum += pot.get_total_bet_amount(player_id);
        }


        Ok(bet_amount - sum)
    }

    pub fn get_summary(&self) -> Vec<HashMap<String, u128>> {
        let mut total:Vec<Pot> = vec![];

        for p in self.list.iter() {
            total.push(p.clone());
        }

        for p in self.closed.iter() {
            total.push(p.clone());
        }

        let mut grouped_pots:Vec<HashMap<String, u128>> = vec![];
        for _ in 0..self.players.len() {
            grouped_pots.push(HashMap::new());
        }

        for p in total.iter() {
            for index in 0..self.players.len() {
                let length = index + 1;
                if p.balance.len() == length {
                    for (player_id, amount) in p.balance.clone().iter() {
                        let mut g = grouped_pots[index].clone();
                        if let Some(v) = g.get(player_id.as_str()) {
                            g.insert(player_id.to_string(), amount.clone() + v.clone());
                        } else {
                            g.insert(player_id.to_string(), amount.clone());
                        }
                        grouped_pots[index] = g;
                    }
                }
            }
        }

        grouped_pots
    }
}