use crate::card::{Card, CardNumber};
use crate::msgs::{GameError, GameResult};

pub fn royal_straight_flush(cards:&Vec<Card>) -> Result<u128, GameError> {
    let base_score = GameResult::RoyalStraightFlush as u128;
    let mut score:u128 = 0;
    //무늬가 같고 A,K,Q,J,10

    let grouped = make_suit_slot(cards);
    for group in grouped.iter() {

        if group.len() >= 5 {
            let mut sorted = group.clone();
            sorted.sort_by_key(|card| card.clone().number as u8);
            sorted.reverse();

            if sorted[0].number == CardNumber::Ace &&
                sorted[4].number == CardNumber::Ten {
                //royal straight flush
                score = base_score;
                break;
            }
        }
    }

    Ok(score)
}

pub fn straight_flush(cards:&Vec<Card>) -> Result<u128, GameError> {
    let base_score = GameResult::StraightFlush as u128;
    let mut score = 0;
    //무늬가 같고 숫자가 안결된것.
    // let mut sorted = cards.clone().sort_by_key(|card| card.clone().number as u8);

    let grouped = make_suit_slot(cards);

    for group in grouped.iter() {
        if group.len() < 5 {
            continue;
        }

        if group[group.len() - 1].number == CardNumber::Ace &&
            group[0].number == CardNumber::Two &&
            group[1].number == CardNumber::Three &&
            group[2].number == CardNumber::Four &&
            group[3].number == CardNumber::Five {
            //a, 2, 3, 4, 5 인경우는 별도처리. a의 값이 14이기 때문

            score = base_score;
            score += group[3].clone().number as u128;
            break;
        } else {
            for i in 0..(group.len() - 4) {
                let index = group.len() - i - 1;
                //높은 거부터 판별하기 위해.

                if (group[index].clone().number as u8 - group[index - 4].clone().number as u8) == 4 {
                    score = base_score;
                    score += group[index].clone().number as u128;
                    break;
                }

            }
        }
    }

    Ok(score)
}

pub fn four_of_a_kind(cards:&Vec<Card>) -> Result<u128, GameError> {
    let base_score = GameResult::FourOfAKind as u128;
    let mut score:u128 = 0;
    //같은 숫자가 4장인 경우. 동시에 포카드인 경우 마지막 5번째 카드(킥커)로 승부를 정함.

    let grouped:Vec<Vec<Card>> = make_number_slot(cards);

    for group in grouped.iter() {
        if group.len() == 4 {
            //숫자가 같은 모양은 4개밖에 없다.
            score += base_score;
            score += group[0].clone().number as u128; //여러 포카드가 발생한 경우 우선순위를 가리기 위해
        }
    }

    Ok(score)
}

pub fn full_house(cards:&Vec<Card>) -> Result<u128, GameError> {
    let base_score = GameResult::FullHouse as u128;
    let mut score:u128 = 0;
    //같은 숫자가 3장(Three of a kind) + 같은 숫자 2장(Pair) 조합.

    let grouped:Vec<Vec<Card>> = make_number_slot(cards);

    let mut group3:Option<u128> = None;
    let mut group2:Option<u128> = None;

    for i in 0..grouped.len() {
        let index = grouped.len() - i - 1;
        let group = &grouped[index];
        //숫자가 높은 카드부터 살펴봄.

        if group.len() >= 2 {
            if group3.is_none() && group.len() >= 3 {
                group3 = Some(index as u128);
            } else if group2.is_none() {
                group2 = Some(index as u128);
            }
        }
    }

    if group3.is_some() && group2.is_some() {
        //풀하우스인 경우.
        score += base_score;
        score += group3.unwrap() * 10000;
        score += group2.unwrap() * 100;
    }

    Ok(score)
}

pub fn flush(cards:&Vec<Card>) -> Result<u128, GameError> {
    let base_score = GameResult::Flush as u128;
    let mut score = 0;
    //5장의 카드가 무늬가 같은 경우.

    let mut flush = vec![];
    let mut other = vec![];
    let grouped:Vec<Vec<Card>> = make_suit_slot(cards);
    for group in &grouped {
        if group.len() >= 5 {
            //상위 5개를 자른다.
            let split = group.split_at(group.len() - 5);
            other = split.0.to_vec();
            flush = split.1.to_vec();

            score += base_score;
            score += flush[4].clone().number as u128 * 100000000;
            score += flush[3].clone().number as u128 * 1000000;
            score += flush[2].clone().number as u128 * 10000;
            score += flush[1].clone().number as u128 * 100;
            score += flush[0].clone().number as u128 * 1;
        }
    }

    Ok(score)
}

pub fn straight(cards:&Vec<Card>) -> Result<u128, GameError> {
    let base_score = GameResult::Straight as u128;
    let mut score:u128 = 0;
    //5장의 카드 숫자가 연속된 경우.

    let mut sorted:Vec<Card> = vec![];
    for item in make_number_slot(cards).iter() {
        if item.len() > 0 {
            sorted.push(item[0].clone());
        }
    }
    sorted.sort_by_key(|card| card.clone().number as u8);

    let case1 = sorted.len() == 7 && (sorted[6].clone().number as u128 - sorted[2].clone().number as u128) == 4;
    let case2 = sorted.len() >= 6 && (sorted[5].clone().number as u128 - sorted[1].clone().number as u128) == 4;
    let case3 = sorted.len() >= 5 && (sorted[4].clone().number as u128 - sorted[0].clone().number as u128) == 4;
    let mut case4 = false;

    if sorted.len() >= 5 &&
        sorted[sorted.len() - 1].number == CardNumber::Ace &&
        sorted[0].number == CardNumber::Two &&
        sorted[1].number == CardNumber::Three &&
        sorted[2].number == CardNumber::Four &&
        sorted[3].number == CardNumber::Five {
        //a,2,3,4,5 를 판별
        case4 = true;
    }

    if case1 || case2 || case3 || case4 {
        //스트레이트인 경우.
        score += base_score;

        if case1 {
            //2,3,4,5,6 번째 카드로 스트레이트
            //6 번째 카드로 우선순위를 가른다.
            score += sorted[6].clone().number as u128;
        } else if case2 {
            //1,2,3,4,5 번째 카드로 스트레이트
            //5 번째 카드로 우선순위를 가른다.
            score += sorted[5].clone().number as u128;
        } else if case3 {
            //0,1,2,3,4 번째 카드로 스트레이트
            //4 번째 카드로 우선순위를 가른다.
            score += sorted[4].clone().number as u128;
        } else if case4 {
            //A,2,3,4,5는 가장 마지막에 체크.
            score += sorted[3].clone().number as u128; // 5
        }
    }

    Ok(score)
}

pub fn three_of_a_kind(cards:&Vec<Card>) -> Result<u128, GameError> {
    let base_score = GameResult::ThreeOfAKind as u128;
    let mut score:u128 = 0;
    //5장의 카드 중 같은 숫자의 카드가 3장인 경우.

    let grouped = make_number_slot(cards);
    for i in 0..grouped.len() {
        //높은 카드부터 3장 여부를 판별한다.
        let index = grouped.len() - i - 1;
        let group = &grouped[index];
        if group.len() == 3 {
            //4장일 경우는 포카드에서 걸러지므로 고려하지 않는다.

            score = base_score;
            score += group[0].clone().number as u128; //트리플 간의 우열을 가리기 위해.
            break;
        }
    }

    Ok(score)
}

pub fn two_pair(cards:&Vec<Card>) -> Result<u128, GameError> {
    let base_score = GameResult::TwoPair as u128;
    let mut score:u128 = 0;
    //5장의 카드 중 같은 숫자 2가지 숫자가 페어인 경우.

    let grouped = make_number_slot(cards);

    let mut pair1:Option<u8> = None;
    let mut pair2:Option<u8> = None;
    for i in 0..grouped.len() {
        //높은 카드부터 3장 여부를 판별한다.
        let index = grouped.len() - i - 1;
        let group = &grouped[index];
        if group.len() == 2 {
            if pair1.is_none() {
                pair1 = Some(index as u8);
            } else if pair2.is_none() {
                pair2 = Some(index as u8);
            }
        }
    }

    if pair1.is_some() && pair2.is_some() {
        //투페어 경우.
        score += base_score;
        score += pair1.unwrap() as u128 * 10000;
        score += pair2.unwrap() as u128 * 100;

        for i in 0..grouped.len() {
            let index = (grouped.len() - i - 1) as u128;
            let group = &grouped[index as usize];
            if group.len() > 0 && index != pair1.unwrap() as u128 && index != pair2.unwrap() as u128 {
                //여러 투페어가 발생한 경우 우선순위를 가리기 위해
                score += index;
                break;
            }
        }
    }

    Ok(score)
}

pub fn one_pair(cards:&Vec<Card>) -> Result<u128, GameError> {
    let base_score = GameResult::OnePair as u128;
    let mut score:u128 = 0;
    //5장의 카드 중 같은 숫자 2가지 숫자가 페어인 경우.


    let grouped = make_number_slot(cards);

    for group in grouped.iter() {
        if group.len() == 2 {
            score += base_score;
            score += group[0].number.clone() as u128 * 100000000;
        }
    }

    if score > 0 {
        //원페어 경우.
        let mut count = 1000000;
        for i in 0..grouped.len() {
            let index = grouped.len() - i - 1;
            let length = grouped[index].len();

            if length == 1 {
                //여러 원페어가 발생한 경우 우선순위를 가리기 위해
                score += index as u128 * count;
                count = count / 100;
            }

            if count == 1 {
                break;
            }
        }
    }

    Ok(score)
}

pub fn high_card(cards:&Vec<Card>) -> Result<u128, GameError> {
    let base_score = GameResult::HighCard as u128;
    //카드가 숫자, 무늬 모두 다른 경우.
    let mut score:u128 = 0;
    let grouped = make_number_slot(cards);
    for i in 0..grouped.len() {
        //높은 카드부터 3장 여부를 판별한다.
        let index = grouped.len() - i - 1;
        let group = &grouped[index];
        if group.len() > 0 {
            score = base_score;
            score += index as u128;
            break;
        }
    }

    Ok(score)
}

pub fn make_suit_slot(cards: &Vec<Card>) -> Vec<Vec<Card>> {
    let mut grouped:Vec<Vec<Card>> = vec![];
    for _ in 0..4 {
        grouped.push(vec![])
    }

    for card in cards.iter() {
        grouped[card.clone().suit as usize].push(card.clone());
    }

    for group in &mut grouped {
        group.sort_by_key(|card| card.clone().number as u8);
    }

    grouped
}

pub fn make_number_slot(cards:&Vec<Card>) -> Vec<Vec<Card>> {
    let mut grouped:Vec<Vec<Card>> = vec![];
    for _ in 0..15 {
        grouped.push(vec![])
    }

    for card in cards.iter() {
        grouped[card.clone().number as usize].push(card.clone());
    }

    grouped
}