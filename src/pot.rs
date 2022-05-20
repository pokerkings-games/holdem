use std::collections::HashMap;
use crate::msgs::GameError;

#[derive(Debug, PartialEq, Clone)]
pub struct Pot {
    pub balance: HashMap<String, u128>,
    pub max_amount: Option<u128>,
}

impl Pot {
    pub fn get_total_bet_amount(&self, player:&String) -> u128 {
        self.balance.get(player.as_str()).unwrap_or(&0u128).clone()
    }

    pub fn get_total_pot_balance(&self) -> u128 {
        let mut sum:u128 = 0;
        for (_, amount) in self.balance.iter() {
            sum += amount.clone();
        }
        sum
    }

    pub fn add_amount(&mut self, player: &String, amount: u128) -> Result<(), GameError>{
        let amount = amount + self.get_total_bet_amount(player);
        self.balance.insert(player.to_string(), amount);

        Ok(())
    }

    pub fn get_overflow_amount(&mut self) -> HashMap<String, u128> {
        let mut overflow_amount = HashMap::new();

        if let Some(max_amount) = self.max_amount {
            let mut keys = vec![];
            for (player_id, amount) in self.balance.iter() {
                if amount.clone() > max_amount {
                    keys.push(player_id.clone());
                    overflow_amount.insert(player_id.to_string(), amount.clone() - max_amount);
                }
            }

            for k in keys.iter() {
                self.balance.insert(k.to_string(), max_amount);
            }

        }

        overflow_amount
    }
}


