use crate::card::{Card};
use crate::examine::examine::{flush, four_of_a_kind, full_house, high_card, one_pair, royal_straight_flush, straight, straight_flush, three_of_a_kind, two_pair};
use crate::msgs::{GameError, GameResult};
use crate::player::Player;
use crate::pots::Pots;


fn get_result(pots: Pots, community_cards:Vec<Card>) -> Result<Vec<(String, GameResult, u128)>, GameError> {
    let mut pot_list = vec![];
    for p in pots.list.iter() {
        pot_list.push(p.clone())
    }
    for p in pots.closed.iter() {
        pot_list.push(p.clone())
    }

    let mut winners:Vec<(String, GameResult, u128)> = vec![];

    for pot in pot_list {
        let mut player_list = vec![];
        for (player_id, amount) in pot.balance.iter() {
            let player = pots.get_player(player_id)?;
            if !player.fold {
                player_list.push((player_id.clone(), player.clone().cards))
            }
        }
        let result = get_winner(player_list, community_cards.clone())?;
        let pot_winner_amount = pot.get_total_pot_balance() / result.len() as u128;
        for winner in result.iter() {
            let mut found = false;
            for w in &mut winners {
                if w.0.clone() == winner.0.clone() {
                    found = true;
                    w.2 += pot_winner_amount;
                }
            }

            if !found {
                winners.push((
                    winner.0.clone(),
                    winner.1.clone(),
                    pot_winner_amount,
                ));
            }

        }
    }

    Ok(winners)
}

fn get_winner(player_cards:Vec<(String, Vec<Card>)>, community_cards:Vec<Card>) -> Result<Vec<(String, GameResult, u128)>, GameError> {
    let mut score:Vec<(String, GameResult, u128)> = vec![];
    for (player_id, player_cards) in player_cards.iter() {
        let mut cards = vec![];
        for c in player_cards.iter() {
            cards.push(c.clone());
        }
        for c in community_cards.iter() {
            cards.push(c.clone());
        }

        let result = get_score(cards)?;
        score.push((player_id.clone(), result.0, result.1));
    }
    score.sort_by_key(|info| info.2);
    score.reverse();

    let mut winners:Vec<(String, GameResult, u128)> = vec![];
    for s in score.iter() {

        if s.2.clone() == score[0].2 {
            winners.push( s.clone());
        }
    }

    Ok(winners)
}

pub fn get_score(cards:Vec<Card>) -> Result<(GameResult, u128), GameError> {
    check_validation(&cards)?;

    let mut result:GameResult = GameResult::RoyalStraightFlush;
    let mut score = royal_straight_flush(&cards)?;
    if score == 0 {
        result = GameResult::StraightFlush;
        score = straight_flush(&cards)?;
    }
    if score == 0 {
        result = GameResult::FourOfAKind;
        score = four_of_a_kind(&cards)?;
    }
    if score == 0 {
        result = GameResult::FullHouse;
        score = full_house(&cards)?;
    }
    if score == 0 {
        result = GameResult::Flush;
        score = flush(&cards)?;
    }
    if score == 0 {
        result = GameResult::Straight;
        score = straight(&cards)?;
    }
    if score == 0 {
        result = GameResult::ThreeOfAKind;
        score = three_of_a_kind(&cards)?;
    }
    if score == 0 {
        result = GameResult::TwoPair;
        score = two_pair(&cards)?;
    }
    if score == 0 {
        result = GameResult::OnePair;
        score = one_pair(&cards)?;
    }
    if score == 0 {
        result = GameResult::HighCard;
        score = high_card(&cards)?;
    }

    Ok((result, score))
}

fn check_validation(cards:&Vec<Card>) -> Result<(), GameError> {
    if cards.len() != 7 {
        return Err(GameError::InvalidCards)
    }

    Ok(())
}