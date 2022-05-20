use crate::card::{Card, CardNumber, CardSuit};
use crate::examine::examine::{flush, four_of_a_kind, full_house, high_card, make_number_slot, make_suit_slot, one_pair, royal_straight_flush, straight, straight_flush, three_of_a_kind, two_pair};
use crate::examine::result::get_score;
use crate::msgs::GameResult;

fn test_get_result() {

    let community_cards = vec![
        Card { suit: CardSuit::Spade, number: CardNumber::Ace },
        Card { suit: CardSuit::Spade, number: CardNumber::Two, },
        Card { suit: CardSuit::Spade, number: CardNumber::King },
        Card { suit: CardSuit::Spade, number: CardNumber::Queen },
        Card { suit: CardSuit::Spade, number: CardNumber::Jack },
    ];

    let player1_cards = vec![
        Card { suit: CardSuit::Spade, number: CardNumber::Ten },
        Card { suit: CardSuit::Spade, number: CardNumber::Three },
    ];

    let player2_cards = vec![
        Card { suit: CardSuit::Spade, number: CardNumber::Ten },
        Card { suit: CardSuit::Spade, number: CardNumber::Three },
    ];

}

#[test]
fn test_get_score() {
    //royal straight flush
    let cards = vec![
        Card { suit: CardSuit::Spade, number: CardNumber::Ace },
        Card { suit: CardSuit::Spade, number: CardNumber::Two, },
        Card { suit: CardSuit::Spade, number: CardNumber::King },
        Card { suit: CardSuit::Spade, number: CardNumber::Queen },
        Card { suit: CardSuit::Spade, number: CardNumber::Jack },
        Card { suit: CardSuit::Spade, number: CardNumber::Nine },
        Card { suit: CardSuit::Spade, number: CardNumber::Ten },
    ];

    let result = get_score(cards).unwrap();
    assert_eq!(result, (GameResult::RoyalStraightFlush, GameResult::RoyalStraightFlush as u128));

    //straight flush
    let cards = vec![
        Card { suit: CardSuit::Spade, number: CardNumber::Ace },
        Card { suit: CardSuit::Spade, number: CardNumber::Two, },
        Card { suit: CardSuit::Spade, number: CardNumber::Three },
        Card { suit: CardSuit::Spade, number: CardNumber::Four },
        Card { suit: CardSuit::Spade, number: CardNumber::Five },
        Card { suit: CardSuit::Spade, number: CardNumber::Nine },
        Card { suit: CardSuit::Spade, number: CardNumber::Ten },
    ];

    let result = get_score(cards).unwrap();
    assert_eq!(result, (GameResult::StraightFlush, GameResult::StraightFlush as u128 + 5));

    //four of a kind
    let cards = vec![
        Card { suit: CardSuit::Spade, number: CardNumber::Ace },
        Card { suit: CardSuit::Heart, number: CardNumber::Ace, },
        Card { suit: CardSuit::Club, number: CardNumber::Ace },
        Card { suit: CardSuit::Diamond, number: CardNumber::Ace },
        Card { suit: CardSuit::Spade, number: CardNumber::Two },
        Card { suit: CardSuit::Spade, number: CardNumber::King },
        Card { suit: CardSuit::Spade, number: CardNumber::Nine },
    ];
    let result = get_score(cards).unwrap();
    assert_eq!(result, (GameResult::FourOfAKind, GameResult::FourOfAKind as u128 + 14));

    //fullhouse
    let cards = vec![
        Card { suit: CardSuit::Spade, number: CardNumber::Seven },
        Card { suit: CardSuit::Heart, number: CardNumber::Seven, },
        Card { suit: CardSuit::Club, number: CardNumber::Seven },
        Card { suit: CardSuit::Diamond, number: CardNumber::Three },
        Card { suit: CardSuit::Spade, number: CardNumber::Three },
        Card { suit: CardSuit::Heart, number: CardNumber::Ace },
        Card { suit: CardSuit::Club, number: CardNumber::Ace },
    ];
    let result = get_score(cards).unwrap();
    assert_eq!(result, (GameResult::FullHouse, GameResult::FullHouse as u128 + 70000 + 1400));

    //flush
    let cards = vec![
        Card { suit: CardSuit::Heart, number: CardNumber::Ace },
        Card { suit: CardSuit::Heart, number: CardNumber::Two, },
        Card { suit: CardSuit::Heart, number: CardNumber::Nine },
        Card { suit: CardSuit::Heart, number: CardNumber::Four },
        Card { suit: CardSuit::Heart, number: CardNumber::Eight },
        Card { suit: CardSuit::Spade, number: CardNumber::Six },
        Card { suit: CardSuit::Spade, number: CardNumber::Seven },
    ];
    let result = get_score(cards).unwrap();
    assert_eq!(result, (GameResult::Flush, GameResult::Flush as u128 + 14*100000000 + 9*1000000 + 8*10000 + 4*100 + 2*1));

    //straight
    let cards = vec![
        Card { suit: CardSuit::Heart, number: CardNumber::Four },
        Card { suit: CardSuit::Spade, number: CardNumber::Five },
        Card { suit: CardSuit::Heart, number: CardNumber::Six },
        Card { suit: CardSuit::Spade, number: CardNumber::Seven, },
        Card { suit: CardSuit::Heart, number: CardNumber::Eight },
        Card { suit: CardSuit::Spade, number: CardNumber::Jack },
        Card { suit: CardSuit::Heart, number: CardNumber::Queen },
    ];
    let result = get_score(cards).unwrap();
    assert_eq!(result, (GameResult::Straight, GameResult::Straight as u128 + 8));

    //three of a kind
    let cards = vec![
        Card { suit: CardSuit::Heart, number: CardNumber::Ace },
        Card { suit: CardSuit::Spade, number: CardNumber::Ace },
        Card { suit: CardSuit::Diamond, number: CardNumber::Ace },
        Card { suit: CardSuit::Heart, number: CardNumber::Nine, },
        Card { suit: CardSuit::Diamond, number: CardNumber::Three },
        Card { suit: CardSuit::Heart, number: CardNumber::Four },
        Card { suit: CardSuit::Diamond, number: CardNumber::Five },
    ];
    let result = get_score(cards).unwrap();
    assert_eq!(result, (GameResult::ThreeOfAKind, GameResult::ThreeOfAKind as u128 + 14));

    //two pair
    let cards = vec![
        Card { suit: CardSuit::Heart, number: CardNumber::Seven },
        Card { suit: CardSuit::Spade, number: CardNumber::Seven },
        Card { suit: CardSuit::Heart, number: CardNumber::Three },
        Card { suit: CardSuit::Spade, number: CardNumber::Three, },
        Card { suit: CardSuit::Heart, number: CardNumber::Nine },
        Card { suit: CardSuit::Spade, number: CardNumber::Nine },
        Card { suit: CardSuit::Heart, number: CardNumber::King },
    ];
    let result = get_score(cards).unwrap();
    assert_eq!(result, (GameResult::TwoPair, GameResult::TwoPair as u128 + 9*10000 + 7*100 + 13*1));

    //one pair
    let cards = vec![
        Card { suit: CardSuit::Heart, number: CardNumber::Seven },
        Card { suit: CardSuit::Spade, number: CardNumber::Seven },
        Card { suit: CardSuit::Heart, number: CardNumber::Ace },
        Card { suit: CardSuit::Spade, number: CardNumber::Two, },
        Card { suit: CardSuit::Heart, number: CardNumber::Three },
        Card { suit: CardSuit::Spade, number: CardNumber::Four },
        Card { suit: CardSuit::Heart, number: CardNumber::Six },
    ];
    let result = get_score(cards).unwrap();
    assert_eq!(result, (GameResult::OnePair, GameResult::OnePair as u128 + 7*100000000 + 14*1000000 + 6*10000 + 4*100));

    let cards = vec![
        Card { suit: CardSuit::Heart, number: CardNumber::Ace },
        Card { suit: CardSuit::Spade, number: CardNumber::Two, },
        Card { suit: CardSuit::Heart, number: CardNumber::Three },
        Card { suit: CardSuit::Spade, number: CardNumber::Four },
        Card { suit: CardSuit::Heart, number: CardNumber::Six },
        Card { suit: CardSuit::Heart, number: CardNumber::Seven },
        Card { suit: CardSuit::Spade, number: CardNumber::Eight },
    ];
    let result = get_score(cards).unwrap();
    assert_eq!(result, (GameResult::HighCard, GameResult::HighCard as u128 + 14));
}
