use crate::card::{Card, CardNumber, CardSuit};
use crate::examine::examine::{flush, four_of_a_kind, full_house, high_card, make_number_slot, make_suit_slot, one_pair, royal_straight_flush, straight, straight_flush, three_of_a_kind, two_pair};
use crate::msgs::GameResult;

#[test]
fn test_make_suit_slot() {
    let cards = vec![
        Card { suit: CardSuit::Spade, number: CardNumber::Ace },
        Card { suit: CardSuit::Heart, number: CardNumber::Two, },
        Card { suit: CardSuit::Diamond, number: CardNumber::King },
        Card { suit: CardSuit::Club, number: CardNumber::Queen },
        Card { suit: CardSuit::Spade, number: CardNumber::Jack },
        Card { suit: CardSuit::Heart, number: CardNumber::Nine },
        Card { suit: CardSuit::Spade, number: CardNumber::Ten },
    ];

    let result = make_suit_slot(&cards);
    assert_eq!(result, vec![
        vec![
            Card { suit: CardSuit::Diamond, number: CardNumber::King },
        ],
        vec![
            Card { suit: CardSuit::Spade, number: CardNumber::Ten },
            Card { suit: CardSuit::Spade, number: CardNumber::Jack },
            Card { suit: CardSuit::Spade, number: CardNumber::Ace },
        ],
        vec![
            Card { suit: CardSuit::Club, number: CardNumber::Queen },
        ],
        vec![
            Card { suit: CardSuit::Heart, number: CardNumber::Two, },
            Card { suit: CardSuit::Heart, number: CardNumber::Nine },
        ],
    ])
}

#[test]
fn test_make_number_slot() {
    let cards = vec![
        Card { suit: CardSuit::Spade, number: CardNumber::Ace },
        Card { suit: CardSuit::Heart, number: CardNumber::Ace, },
        Card { suit: CardSuit::Diamond, number: CardNumber::King },
        Card { suit: CardSuit::Club, number: CardNumber::Queen },
        Card { suit: CardSuit::Spade, number: CardNumber::King },
        Card { suit: CardSuit::Heart, number: CardNumber::Nine },
        Card { suit: CardSuit::Spade, number: CardNumber::Ten },
    ];

    let result = make_number_slot(&cards);
    assert_eq!(result, vec![
        vec![
        ],
        vec![
        ],
        vec![
        ],
        vec![
        ],
        vec![
        ],
        vec![
        ],
        vec![
        ],
        vec![
        ],
        vec![
        ],
        vec![
            Card { suit: CardSuit::Heart, number: CardNumber::Nine },
        ],
        vec![
            Card { suit: CardSuit::Spade, number: CardNumber::Ten },
        ],
        vec![
        ],
        vec![
            Card { suit: CardSuit::Club, number: CardNumber::Queen },
        ],
        vec![
            Card { suit: CardSuit::Diamond, number: CardNumber::King },
            Card { suit: CardSuit::Spade, number: CardNumber::King },
        ],
        vec![
            Card { suit: CardSuit::Spade, number: CardNumber::Ace },
            Card { suit: CardSuit::Heart, number: CardNumber::Ace, },
        ],
    ])
}

#[test]
fn test_royal_straight_flush() {
    let cards = vec![
        Card { suit: CardSuit::Spade, number: CardNumber::Ace },
        Card { suit: CardSuit::Spade, number: CardNumber::Two, },
        Card { suit: CardSuit::Spade, number: CardNumber::King },
        Card { suit: CardSuit::Spade, number: CardNumber::Queen },
        Card { suit: CardSuit::Spade, number: CardNumber::Jack },
        Card { suit: CardSuit::Spade, number: CardNumber::Nine },
        Card { suit: CardSuit::Spade, number: CardNumber::Ten },
    ];

    let result = royal_straight_flush(&cards).unwrap();
    assert_eq!(result, GameResult::RoyalStraightFlush as u128);

    let cards = vec![
        Card { suit: CardSuit::Spade, number: CardNumber::Ace },
        Card { suit: CardSuit::Heart, number: CardNumber::Two, },
        Card { suit: CardSuit::Spade, number: CardNumber::King },
        Card { suit: CardSuit::Spade, number: CardNumber::Queen },
        Card { suit: CardSuit::Spade, number: CardNumber::Jack },
        Card { suit: CardSuit::Spade, number: CardNumber::Nine },
        Card { suit: CardSuit::Spade, number: CardNumber::Ten },
    ];

    let result = royal_straight_flush(&cards).unwrap();
    assert_eq!(result, GameResult::RoyalStraightFlush as u128);

    let cards = vec![
        Card { suit: CardSuit::Heart, number: CardNumber::Ace },
        Card { suit: CardSuit::Heart, number: CardNumber::Two, },
        Card { suit: CardSuit::Spade, number: CardNumber::King },
        Card { suit: CardSuit::Spade, number: CardNumber::Queen },
        Card { suit: CardSuit::Spade, number: CardNumber::Jack },
        Card { suit: CardSuit::Spade, number: CardNumber::Nine },
        Card { suit: CardSuit::Spade, number: CardNumber::Ten },
    ];

    let result = royal_straight_flush(&cards).unwrap();
    assert_eq!(result, 0)
}

#[test]
fn test_straight_flush() {

    //a,2,3,4,5,6,7
    let cards = vec![
        Card { suit: CardSuit::Spade, number: CardNumber::Ace },
        Card { suit: CardSuit::Spade, number: CardNumber::Two, },
        Card { suit: CardSuit::Spade, number: CardNumber::Three },
        Card { suit: CardSuit::Spade, number: CardNumber::Four },
        Card { suit: CardSuit::Spade, number: CardNumber::Five },
        Card { suit: CardSuit::Spade, number: CardNumber::Nine },
        Card { suit: CardSuit::Spade, number: CardNumber::Ten },
    ];
    let result = straight_flush(&cards).unwrap();
    assert_eq!(result, GameResult::StraightFlush as u128 + 5);

    //2,3,4,5,5,6,7
    let cards = vec![
        Card { suit: CardSuit::Spade, number: CardNumber::Two, },
        Card { suit: CardSuit::Spade, number: CardNumber::Three },
        Card { suit: CardSuit::Spade, number: CardNumber::Four },
        Card { suit: CardSuit::Spade, number: CardNumber::Five },
        Card { suit: CardSuit::Heart, number: CardNumber::Five },
        Card { suit: CardSuit::Spade, number: CardNumber::Six },
        Card { suit: CardSuit::Spade, number: CardNumber::Seven },
    ];

    let result = straight_flush(&cards).unwrap();
    assert_eq!(result, GameResult::StraightFlush as u128 + 7);

    //10, j, q, k, a
    let cards = vec![
        Card { suit: CardSuit::Spade, number: CardNumber::Ace },
        Card { suit: CardSuit::Spade, number: CardNumber::Two, },
        Card { suit: CardSuit::Spade, number: CardNumber::King },
        Card { suit: CardSuit::Spade, number: CardNumber::Queen },
        Card { suit: CardSuit::Spade, number: CardNumber::Jack },
        Card { suit: CardSuit::Spade, number: CardNumber::Nine },
        Card { suit: CardSuit::Spade, number: CardNumber::Ten },
    ];

    let result = straight_flush(&cards).unwrap();
    assert_eq!(result, GameResult::StraightFlush as u128 + 14);

    let cards = vec![
        Card { suit: CardSuit::Heart, number: CardNumber::Ace },
        Card { suit: CardSuit::Heart, number: CardNumber::Two, },
        Card { suit: CardSuit::Spade, number: CardNumber::Ace },
        Card { suit: CardSuit::Heart, number: CardNumber::Two },
        Card { suit: CardSuit::Spade, number: CardNumber::Three },
        Card { suit: CardSuit::Spade, number: CardNumber::Four },
        Card { suit: CardSuit::Spade, number: CardNumber::Five },
    ];

    let result = straight_flush(&cards).unwrap();
    assert_eq!(result, 0)
}

#[test]
fn test_four_of_a_kind() {

    //a,a,a,a,2,k,9
    let cards = vec![
        Card { suit: CardSuit::Spade, number: CardNumber::Ace },
        Card { suit: CardSuit::Heart, number: CardNumber::Ace, },
        Card { suit: CardSuit::Club, number: CardNumber::Ace },
        Card { suit: CardSuit::Diamond, number: CardNumber::Ace },
        Card { suit: CardSuit::Spade, number: CardNumber::Two },
        Card { suit: CardSuit::Spade, number: CardNumber::King },
        Card { suit: CardSuit::Spade, number: CardNumber::Nine },
    ];

    let result = four_of_a_kind(&cards).unwrap();
    assert_eq!(result, GameResult::FourOfAKind as u128 + 14);

    //2,2,2,2,j,9,10
    let cards = vec![
        Card { suit: CardSuit::Spade, number: CardNumber::Two },
        Card { suit: CardSuit::Heart, number: CardNumber::Two, },
        Card { suit: CardSuit::Club, number: CardNumber::Two },
        Card { suit: CardSuit::Diamond, number: CardNumber::Two },
        Card { suit: CardSuit::Spade, number: CardNumber::Jack },
        Card { suit: CardSuit::Spade, number: CardNumber::Nine },
        Card { suit: CardSuit::Spade, number: CardNumber::Ten },
    ];

    let result = four_of_a_kind(&cards).unwrap();
    assert_eq!(result, GameResult::FourOfAKind as u128 + 2);

    let cards = vec![
        Card { suit: CardSuit::Heart, number: CardNumber::Ace },
        Card { suit: CardSuit::Heart, number: CardNumber::Two, },
        Card { suit: CardSuit::Spade, number: CardNumber::Ace },
        Card { suit: CardSuit::Diamond, number: CardNumber::Two },
        Card { suit: CardSuit::Spade, number: CardNumber::Three },
        Card { suit: CardSuit::Spade, number: CardNumber::Four },
        Card { suit: CardSuit::Spade, number: CardNumber::Five },
    ];

    let result = four_of_a_kind(&cards).unwrap();
    assert_eq!(result, 0)
}

#[test]
fn test_full_house() {

    // 7777333
    let cards = vec![
        Card { suit: CardSuit::Spade, number: CardNumber::Seven },
        Card { suit: CardSuit::Heart, number: CardNumber::Seven, },
        Card { suit: CardSuit::Club, number: CardNumber::Seven },
        Card { suit: CardSuit::Diamond, number: CardNumber::Seven },
        Card { suit: CardSuit::Spade, number: CardNumber::Three },
        Card { suit: CardSuit::Heart, number: CardNumber::Three },
        Card { suit: CardSuit::Club, number: CardNumber::Three },
    ];

    let result = full_house(&cards).unwrap();
    assert_eq!(result, GameResult::FullHouse as u128 + 70000 + 300 );

    // 7773333
    let cards = vec![
        Card { suit: CardSuit::Spade, number: CardNumber::Seven },
        Card { suit: CardSuit::Heart, number: CardNumber::Seven, },
        Card { suit: CardSuit::Club, number: CardNumber::Seven },
        Card { suit: CardSuit::Diamond, number: CardNumber::Three },
        Card { suit: CardSuit::Spade, number: CardNumber::Three },
        Card { suit: CardSuit::Heart, number: CardNumber::Three },
        Card { suit: CardSuit::Club, number: CardNumber::Three },
    ];

    let result = full_house(&cards).unwrap();
    assert_eq!(result, GameResult::FullHouse as u128 + 70000 + 300 );

    // 77733AA
    let cards = vec![
        Card { suit: CardSuit::Spade, number: CardNumber::Seven },
        Card { suit: CardSuit::Heart, number: CardNumber::Seven, },
        Card { suit: CardSuit::Club, number: CardNumber::Seven },
        Card { suit: CardSuit::Diamond, number: CardNumber::Three },
        Card { suit: CardSuit::Spade, number: CardNumber::Three },
        Card { suit: CardSuit::Heart, number: CardNumber::Ace },
        Card { suit: CardSuit::Club, number: CardNumber::Ace },
    ];

    let result = full_house(&cards).unwrap();
    assert_eq!(result, GameResult::FullHouse as u128 + 70000 + 1400 );

    // 77733AK
    let cards = vec![
        Card { suit: CardSuit::Spade, number: CardNumber::Seven },
        Card { suit: CardSuit::Heart, number: CardNumber::Seven, },
        Card { suit: CardSuit::Club, number: CardNumber::Seven },
        Card { suit: CardSuit::Diamond, number: CardNumber::Three },
        Card { suit: CardSuit::Spade, number: CardNumber::Three },
        Card { suit: CardSuit::Heart, number: CardNumber::Ace },
        Card { suit: CardSuit::Club, number: CardNumber::King },
    ];

    let result = full_house(&cards).unwrap();
    assert_eq!(result, GameResult::FullHouse as u128 + 70000 + 300 );

    // 77333AA
    let cards = vec![
        Card { suit: CardSuit::Spade, number: CardNumber::Seven },
        Card { suit: CardSuit::Heart, number: CardNumber::Seven, },
        Card { suit: CardSuit::Club, number: CardNumber::Three },
        Card { suit: CardSuit::Diamond, number: CardNumber::Three },
        Card { suit: CardSuit::Spade, number: CardNumber::Three },
        Card { suit: CardSuit::Heart, number: CardNumber::Ace },
        Card { suit: CardSuit::Club, number: CardNumber::Ace },
    ];

    let result = full_house(&cards).unwrap();
    assert_eq!(result, GameResult::FullHouse as u128 + 30000 + 1400 );

    // 77333AK
    let cards = vec![
        Card { suit: CardSuit::Spade, number: CardNumber::Seven },
        Card { suit: CardSuit::Heart, number: CardNumber::Seven, },
        Card { suit: CardSuit::Club, number: CardNumber::Three },
        Card { suit: CardSuit::Diamond, number: CardNumber::Three },
        Card { suit: CardSuit::Spade, number: CardNumber::Three },
        Card { suit: CardSuit::Heart, number: CardNumber::Ace },
        Card { suit: CardSuit::Club, number: CardNumber::King },
    ];

    let result = full_house(&cards).unwrap();
    assert_eq!(result, GameResult::FullHouse as u128 + 30000 + 700 );

    // 73333AK - fail
    let cards = vec![
        Card { suit: CardSuit::Spade, number: CardNumber::Seven },
        Card { suit: CardSuit::Heart, number: CardNumber::Three, },
        Card { suit: CardSuit::Club, number: CardNumber::Three },
        Card { suit: CardSuit::Diamond, number: CardNumber::Three },
        Card { suit: CardSuit::Spade, number: CardNumber::Three },
        Card { suit: CardSuit::Heart, number: CardNumber::Ace },
        Card { suit: CardSuit::Club, number: CardNumber::King },
    ];

    let result = full_house(&cards).unwrap();
    assert_eq!(result, 0 );
}

#[test]
fn test_flush() {

    // heart * 5 , spade * 2
    let cards = vec![
        Card { suit: CardSuit::Heart, number: CardNumber::Ace },
        Card { suit: CardSuit::Heart, number: CardNumber::Two, },
        Card { suit: CardSuit::Heart, number: CardNumber::Three },
        Card { suit: CardSuit::Heart, number: CardNumber::Four },
        Card { suit: CardSuit::Heart, number: CardNumber::Five },
        Card { suit: CardSuit::Spade, number: CardNumber::Six },
        Card { suit: CardSuit::Spade, number: CardNumber::Seven },
    ];

    let result = flush(&cards).unwrap();
    assert_eq!(result, GameResult::Flush as u128 + 14*100000000 + 5*1000000 + 4*10000 + 3*100 + 2*1);

    // heart * 6 , spade * 1
    let cards = vec![
        Card { suit: CardSuit::Heart, number: CardNumber::Ace },
        Card { suit: CardSuit::Heart, number: CardNumber::Two, },
        Card { suit: CardSuit::Heart, number: CardNumber::Three },
        Card { suit: CardSuit::Heart, number: CardNumber::Four },
        Card { suit: CardSuit::Heart, number: CardNumber::Five },
        Card { suit: CardSuit::Heart, number: CardNumber::Six },
        Card { suit: CardSuit::Spade, number: CardNumber::Seven },
    ];

    let result = flush(&cards).unwrap();
    assert_eq!(result, GameResult::Flush as u128 + 14*100000000 + 6*1000000 + 5*10000 + 4*100 + 3*1);

    // heart * 4 , spade * 3
    let cards = vec![
        Card { suit: CardSuit::Heart, number: CardNumber::Ace },
        Card { suit: CardSuit::Heart, number: CardNumber::Two, },
        Card { suit: CardSuit::Heart, number: CardNumber::Three },
        Card { suit: CardSuit::Heart, number: CardNumber::Four },
        Card { suit: CardSuit::Spade, number: CardNumber::Five },
        Card { suit: CardSuit::Spade, number: CardNumber::Six },
        Card { suit: CardSuit::Spade, number: CardNumber::Seven },
    ];

    let result = flush(&cards).unwrap();
    assert_eq!(result, 0);
}

#[test]
fn test_straight() {

    // 4,5,6,7,8,J,Q
    let cards = vec![
        Card { suit: CardSuit::Heart, number: CardNumber::Four },
        Card { suit: CardSuit::Heart, number: CardNumber::Five },
        Card { suit: CardSuit::Heart, number: CardNumber::Six },
        Card { suit: CardSuit::Heart, number: CardNumber::Seven, },
        Card { suit: CardSuit::Heart, number: CardNumber::Eight },
        Card { suit: CardSuit::Heart, number: CardNumber::Jack },
        Card { suit: CardSuit::Heart, number: CardNumber::Queen },
    ];
    let result = straight(&cards).unwrap();
    assert_eq!(result, GameResult::Straight as u128 + 8);

    // 2,4,5,6,7,8,J
    let cards = vec![
        Card { suit: CardSuit::Heart, number: CardNumber::Two, },
        Card { suit: CardSuit::Heart, number: CardNumber::Four },
        Card { suit: CardSuit::Heart, number: CardNumber::Five },
        Card { suit: CardSuit::Heart, number: CardNumber::Six },
        Card { suit: CardSuit::Heart, number: CardNumber::Seven },
        Card { suit: CardSuit::Heart, number: CardNumber::Eight },
        Card { suit: CardSuit::Heart, number: CardNumber::Jack },
    ];
    let result = straight(&cards).unwrap();
    assert_eq!(result, GameResult::Straight as u128 + 8);

    // 2,3,5,6,7,8,9
    let cards = vec![
        Card { suit: CardSuit::Heart, number: CardNumber::Two, },
        Card { suit: CardSuit::Heart, number: CardNumber::Three },
        Card { suit: CardSuit::Heart, number: CardNumber::Five },
        Card { suit: CardSuit::Heart, number: CardNumber::Six },
        Card { suit: CardSuit::Heart, number: CardNumber::Seven },
        Card { suit: CardSuit::Heart, number: CardNumber::Eight },
        Card { suit: CardSuit::Heart, number: CardNumber::Nine },
    ];
    let result = straight(&cards).unwrap();
    assert_eq!(result, GameResult::Straight as u128 + 9);

    // 2,3,4,5,6,7,8
    let cards = vec![
        Card { suit: CardSuit::Heart, number: CardNumber::Two, },
        Card { suit: CardSuit::Heart, number: CardNumber::Three },
        Card { suit: CardSuit::Heart, number: CardNumber::Four },
        Card { suit: CardSuit::Heart, number: CardNumber::Five },
        Card { suit: CardSuit::Heart, number: CardNumber::Six },
        Card { suit: CardSuit::Heart, number: CardNumber::Seven },
        Card { suit: CardSuit::Heart, number: CardNumber::Eight },
    ];
    let result = straight(&cards).unwrap();
    assert_eq!(result, GameResult::Straight as u128 + 8);

    // A,2,3,4,5,6,7
    let cards = vec![
        Card { suit: CardSuit::Heart, number: CardNumber::Ace },
        Card { suit: CardSuit::Heart, number: CardNumber::Two, },
        Card { suit: CardSuit::Heart, number: CardNumber::Three },
        Card { suit: CardSuit::Heart, number: CardNumber::Four },
        Card { suit: CardSuit::Heart, number: CardNumber::Five },
        Card { suit: CardSuit::Heart, number: CardNumber::Six },
        Card { suit: CardSuit::Heart, number: CardNumber::Seven },
    ];
    let result = straight(&cards).unwrap();
    assert_eq!(result, GameResult::Straight as u128 + 7);

    // A,2,3,4,5,7,8
    let cards = vec![
        Card { suit: CardSuit::Heart, number: CardNumber::Ace },
        Card { suit: CardSuit::Heart, number: CardNumber::Two, },
        Card { suit: CardSuit::Heart, number: CardNumber::Three },
        Card { suit: CardSuit::Heart, number: CardNumber::Four },
        Card { suit: CardSuit::Heart, number: CardNumber::Five },
        Card { suit: CardSuit::Heart, number: CardNumber::Seven },
        Card { suit: CardSuit::Heart, number: CardNumber::Eight },
    ];
    let result = straight(&cards).unwrap();
    assert_eq!(result, GameResult::Straight as u128 + 5);

    // 8,9,10,J,Q,K,A
    let cards = vec![
        Card { suit: CardSuit::Heart, number: CardNumber::Eight },
        Card { suit: CardSuit::Heart, number: CardNumber::Nine, },
        Card { suit: CardSuit::Heart, number: CardNumber::Ten },
        Card { suit: CardSuit::Heart, number: CardNumber::Jack },
        Card { suit: CardSuit::Heart, number: CardNumber::Queen },
        Card { suit: CardSuit::Heart, number: CardNumber::King },
        Card { suit: CardSuit::Heart, number: CardNumber::Ace },
    ];
    let result = straight(&cards).unwrap();
    assert_eq!(result, GameResult::Straight as u128 + 14);

    // 7,8,9,J,Q,K,A
    let cards = vec![
        Card { suit: CardSuit::Heart, number: CardNumber::Seven },
        Card { suit: CardSuit::Heart, number: CardNumber::Eight },
        Card { suit: CardSuit::Heart, number: CardNumber::Nine, },
        Card { suit: CardSuit::Heart, number: CardNumber::Jack },
        Card { suit: CardSuit::Heart, number: CardNumber::Queen },
        Card { suit: CardSuit::Heart, number: CardNumber::King },
        Card { suit: CardSuit::Heart, number: CardNumber::Ace },
    ];
    let result = straight(&cards).unwrap();
    assert_eq!(result, 0);
}

fn test_three_of_a_kind() {
    // A,A,A,2,3,4,5
    let cards = vec![
        Card { suit: CardSuit::Heart, number: CardNumber::Ace },
        Card { suit: CardSuit::Spade, number: CardNumber::Ace },
        Card { suit: CardSuit::Diamond, number: CardNumber::Ace },
        Card { suit: CardSuit::Heart, number: CardNumber::Two, },
        Card { suit: CardSuit::Heart, number: CardNumber::Three },
        Card { suit: CardSuit::Heart, number: CardNumber::Four },
        Card { suit: CardSuit::Heart, number: CardNumber::Five },
    ];
    let result = three_of_a_kind(&cards).unwrap();
    assert_eq!(result, GameResult::ThreeOfAKind as u128 + 14);

    // Q,Q,Q,2,2,2,5
    let cards = vec![
        Card { suit: CardSuit::Heart, number: CardNumber::Queen },
        Card { suit: CardSuit::Spade, number: CardNumber::Queen },
        Card { suit: CardSuit::Diamond, number: CardNumber::Queen },
        Card { suit: CardSuit::Heart, number: CardNumber::Two, },
        Card { suit: CardSuit::Spade, number: CardNumber::Two },
        Card { suit: CardSuit::Diamond, number: CardNumber::Two },
        Card { suit: CardSuit::Heart, number: CardNumber::Five },
    ];
    let result = three_of_a_kind(&cards).unwrap();
    assert_eq!(result, GameResult::ThreeOfAKind as u128 + 12);

    // 2,2,2,3,4,5,6
    let cards = vec![
        Card { suit: CardSuit::Heart, number: CardNumber::Two },
        Card { suit: CardSuit::Spade, number: CardNumber::Two },
        Card { suit: CardSuit::Diamond, number: CardNumber::Two },
        Card { suit: CardSuit::Heart, number: CardNumber::Three },
        Card { suit: CardSuit::Heart, number: CardNumber::Four },
        Card { suit: CardSuit::Heart, number: CardNumber::Five },
        Card { suit: CardSuit::Heart, number: CardNumber::Six, },
    ];
    let result = three_of_a_kind(&cards).unwrap();
    assert_eq!(result, GameResult::ThreeOfAKind as u128 + 2);

    // 2,2,3,4,5,6,7 - fail
    let cards = vec![
        Card { suit: CardSuit::Heart, number: CardNumber::Two },
        Card { suit: CardSuit::Spade, number: CardNumber::Two },
        Card { suit: CardSuit::Heart, number: CardNumber::Three },
        Card { suit: CardSuit::Heart, number: CardNumber::Four },
        Card { suit: CardSuit::Heart, number: CardNumber::Five },
        Card { suit: CardSuit::Heart, number: CardNumber::Six, },
        Card { suit: CardSuit::Diamond, number: CardNumber::Seven },
    ];
    let result = three_of_a_kind(&cards).unwrap();
    assert_eq!(result, 0);
}

#[test]
fn test_two_pair() {
    // 7 7 3 3 9 9 K
    let cards = vec![
        Card { suit: CardSuit::Heart, number: CardNumber::Seven },
        Card { suit: CardSuit::Spade, number: CardNumber::Seven },
        Card { suit: CardSuit::Heart, number: CardNumber::Three },
        Card { suit: CardSuit::Spade, number: CardNumber::Three, },
        Card { suit: CardSuit::Heart, number: CardNumber::Nine },
        Card { suit: CardSuit::Spade, number: CardNumber::Nine },
        Card { suit: CardSuit::Heart, number: CardNumber::King },
    ];
    let result = two_pair(&cards).unwrap();
    assert_eq!(result, GameResult::TwoPair as u128 + 9*10000 + 7*100 + 13*1);

    // A A 3 3 9 9 K
    let cards = vec![
        Card { suit: CardSuit::Heart, number: CardNumber::Ace },
        Card { suit: CardSuit::Spade, number: CardNumber::Ace },
        Card { suit: CardSuit::Heart, number: CardNumber::Three },
        Card { suit: CardSuit::Spade, number: CardNumber::Three, },
        Card { suit: CardSuit::Heart, number: CardNumber::Nine },
        Card { suit: CardSuit::Spade, number: CardNumber::Nine },
        Card { suit: CardSuit::Heart, number: CardNumber::King },
    ];
    let result = two_pair(&cards).unwrap();
    assert_eq!(result, GameResult::TwoPair as u128 + 14*10000 + 9*100 + 13*1);

    // 7 7 3 3 9 9 A
    let cards = vec![
        Card { suit: CardSuit::Heart, number: CardNumber::Seven },
        Card { suit: CardSuit::Spade, number: CardNumber::Seven },
        Card { suit: CardSuit::Heart, number: CardNumber::Three },
        Card { suit: CardSuit::Spade, number: CardNumber::Three, },
        Card { suit: CardSuit::Heart, number: CardNumber::Nine },
        Card { suit: CardSuit::Spade, number: CardNumber::Nine },
        Card { suit: CardSuit::Heart, number: CardNumber::Ace },
    ];
    let result = two_pair(&cards).unwrap();
    assert_eq!(result, GameResult::TwoPair as u128 + 9*10000 + 7*100 + 14*1);

    // 1 2 3 4 5 6 7 - fail
    let cards = vec![
        Card { suit: CardSuit::Heart, number: CardNumber::Ace },
        Card { suit: CardSuit::Heart, number: CardNumber::Two },
        Card { suit: CardSuit::Heart, number: CardNumber::Three },
        Card { suit: CardSuit::Heart, number: CardNumber::Four, },
        Card { suit: CardSuit::Heart, number: CardNumber::Five },
        Card { suit: CardSuit::Heart, number: CardNumber::Six },
        Card { suit: CardSuit::Heart, number: CardNumber::Seven },
    ];
    let result = two_pair(&cards).unwrap();
    assert_eq!(result, 0);
}

#[test]
fn test_one_pair() {
    // 7 7 A 2 3 4 5
    let cards = vec![
        Card { suit: CardSuit::Heart, number: CardNumber::Seven },
        Card { suit: CardSuit::Spade, number: CardNumber::Seven },
        Card { suit: CardSuit::Heart, number: CardNumber::Ace },
        Card { suit: CardSuit::Spade, number: CardNumber::Two, },
        Card { suit: CardSuit::Heart, number: CardNumber::Three },
        Card { suit: CardSuit::Spade, number: CardNumber::Four },
        Card { suit: CardSuit::Heart, number: CardNumber::Five },
    ];
    let result = one_pair(&cards).unwrap();
    assert_eq!(result, GameResult::OnePair as u128 + 7*100000000 + 14*1000000 + 5*10000 + 4*100);

    // 7 6 A 2 3 4 5
    let cards = vec![
        Card { suit: CardSuit::Heart, number: CardNumber::Seven },
        Card { suit: CardSuit::Spade, number: CardNumber::Six },
        Card { suit: CardSuit::Heart, number: CardNumber::Ace },
        Card { suit: CardSuit::Spade, number: CardNumber::Two, },
        Card { suit: CardSuit::Heart, number: CardNumber::Three },
        Card { suit: CardSuit::Spade, number: CardNumber::Four },
        Card { suit: CardSuit::Heart, number: CardNumber::Five },
    ];
    let result = one_pair(&cards).unwrap();
    assert_eq!(result, 0);

}

#[test]
fn test_high_card() {
    // A 2 3 4 5 6 7
    let cards = vec![
        Card { suit: CardSuit::Heart, number: CardNumber::Ace },
        Card { suit: CardSuit::Spade, number: CardNumber::Two, },
        Card { suit: CardSuit::Heart, number: CardNumber::Three },
        Card { suit: CardSuit::Spade, number: CardNumber::Four },
        Card { suit: CardSuit::Heart, number: CardNumber::Five },
        Card { suit: CardSuit::Heart, number: CardNumber::Six },
        Card { suit: CardSuit::Heart, number: CardNumber::Seven },
    ];
    let result = high_card(&cards).unwrap();
    assert_eq!(result, GameResult::HighCard as u128 + 14);

    // 2 3 4 5 6 7 8
    let cards = vec![
        Card { suit: CardSuit::Spade, number: CardNumber::Two, },
        Card { suit: CardSuit::Heart, number: CardNumber::Three },
        Card { suit: CardSuit::Spade, number: CardNumber::Four },
        Card { suit: CardSuit::Heart, number: CardNumber::Five },
        Card { suit: CardSuit::Heart, number: CardNumber::Six },
        Card { suit: CardSuit::Heart, number: CardNumber::Seven },
        Card { suit: CardSuit::Heart, number: CardNumber::Eight },
    ];
    let result = high_card(&cards).unwrap();
    assert_eq!(result, GameResult::HighCard as u128 + 8);

}