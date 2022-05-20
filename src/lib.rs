use crate::card::{Card, CardNumber, CardSuit};
use crate::holdem::{Holdem};
use crate::msgs::{Status};
use crate::msgs::ExecuteMsg::{BigBlind, Call, Raise, SetFlop, SetTurn, SmallBlind};
use crate::player::PlayerInitMsg;

// // use crate::card::{Card, CardNumber, CardSuit};
// // use crate::game::{Action, Game, GameError, Status};
// use crate::game::{GameError};
// use crate::pots::{Pots};
// // use crate::player::{Player, PlayerInitMsg};
//
mod card;
mod player;
mod holdem;
mod history;
mod pots;
mod pot;
mod msgs;
mod examine;
mod game;

#[cfg(test)]
mod tests2 {
    use crate::{start};

    #[test]
    fn it_works() {
        start();
        // start_preflop_check();
    }
}

fn start() {
    let cards = vec![
        Card { suit: CardSuit::Club, number: CardNumber::Ace },
        Card { suit: CardSuit::Club, number: CardNumber::Two },
        Card { suit: CardSuit::Club, number: CardNumber::Three },
        Card { suit: CardSuit::Club, number: CardNumber::Four },
        Card { suit: CardSuit::Club, number: CardNumber::Five },
    ];

    let player0 = PlayerInitMsg {
        id: "p0".to_string(),
        balance: 80,
        straddle: false,
        cards: vec![
            Card { suit: CardSuit::Spade, number: CardNumber::Queen },
            Card { suit: CardSuit::Spade, number: CardNumber::King },
        ]
    };

    let player1 = PlayerInitMsg {
        id: "p1".to_string(),
        balance: 90,
        straddle: false,
        cards: vec![
            Card { suit: CardSuit::Spade, number: CardNumber::Ten },
            Card { suit: CardSuit::Spade, number: CardNumber::Jack },
        ]
    };

    let player2 = PlayerInitMsg {
        id: "p2".to_string(),
        balance: 150,
        straddle: false,
        cards: vec![
            Card { suit: CardSuit::Spade, number: CardNumber::Three },
            Card { suit: CardSuit::Spade, number: CardNumber::Four },
        ]
    };

    let player3 = PlayerInitMsg {
        id: "p3".to_string(),
        balance: 100,
        straddle: false,
        cards: vec![
            Card { suit: CardSuit::Heart, number: CardNumber::Five },
            Card { suit: CardSuit::Heart, number: CardNumber::Six },
        ]
    };

    let mut game = Holdem::new(5, 10, vec![player0.clone(), player1.clone(), player2.clone(), player3.clone()]).unwrap();

    //small
    game.execute(SmallBlind {
        player_id: player0.id.to_string(),
    }).unwrap();
    assert_eq!(game.pots.players[0].balance,  75u128);
    assert_eq!(game.pots.players[1].balance,  90u128);
    assert_eq!(game.pots.players[2].balance, 150u128);
    assert_eq!(game.pots.players[3].balance, 100u128);
    assert_eq!(game.pots.list.len(), 1);
    assert_eq!(game.pots.list[0].balance.get("p0").unwrap(), &5);
    assert_eq!(game.pots.closed.len(), 0);

    //big
    game.execute(BigBlind {
        player_id: player1.id.to_string(),
    }).unwrap();
    assert_eq!(game.pots.players[0].balance,  75u128);
    assert_eq!(game.pots.players[1].balance,  80u128);
    assert_eq!(game.pots.players[2].balance, 150u128);
    assert_eq!(game.pots.players[3].balance, 100u128);
    assert_eq!(game.pots.list.len(), 1);
    assert_eq!(game.pots.list[0].balance.get("p0").unwrap(), &5);
    assert_eq!(game.pots.list[0].balance.get("p1").unwrap(), &10);
    assert_eq!(game.pots.closed.len(), 0);

    //call
    game.execute(Call {
        player_id: player2.id.to_string(),
    }).unwrap();
    assert_eq!(game.pots.players[0].balance,  75u128);
    assert_eq!(game.pots.players[1].balance,  80u128);
    assert_eq!(game.pots.players[2].balance, 140u128);
    assert_eq!(game.pots.players[3].balance, 100u128);
    assert_eq!(game.pots.list.len(), 1);
    assert_eq!(game.pots.list[0].balance.get("p0").unwrap(), &5);
    assert_eq!(game.pots.list[0].balance.get("p1").unwrap(), &10);
    assert_eq!(game.pots.list[0].balance.get("p2").unwrap(), &10);
    assert_eq!(game.pots.closed.len(), 0);

    //raise
    game.execute(Raise {
        player_id: player3.id.to_string(),
        amount: 20
    }).unwrap();
    assert_eq!(game.pots.players[0].balance,  75u128);
    assert_eq!(game.pots.players[1].balance,  80u128);
    assert_eq!(game.pots.players[2].balance, 140u128);
    assert_eq!(game.pots.players[3].balance,  80u128);
    assert_eq!(game.pots.list.len(), 1);
    assert_eq!(game.pots.list[0].balance.get("p0").unwrap(), &5);
    assert_eq!(game.pots.list[0].balance.get("p1").unwrap(), &10);
    assert_eq!(game.pots.list[0].balance.get("p2").unwrap(), &10);
    assert_eq!(game.pots.list[0].balance.get("p3").unwrap(), &20);
    assert_eq!(game.pots.closed.len(), 0);

    //call
    game.execute(Call {
        player_id: player0.id.to_string(),
    }).unwrap();
    assert_eq!(game.pots.players[0].balance,  60u128);
    assert_eq!(game.pots.players[1].balance,  80u128);
    assert_eq!(game.pots.players[2].balance, 140u128);
    assert_eq!(game.pots.players[3].balance,  80u128);
    assert_eq!(game.pots.list.len(), 1);
    assert_eq!(game.pots.list[0].balance.get("p0").unwrap(), &20);
    assert_eq!(game.pots.list[0].balance.get("p1").unwrap(), &10);
    assert_eq!(game.pots.list[0].balance.get("p2").unwrap(), &10);
    assert_eq!(game.pots.list[0].balance.get("p3").unwrap(), &20);
    assert_eq!(game.pots.closed.len(), 0);

    //call
    game.execute(Call {
        player_id: player1.id.to_string(),
    }).unwrap();
    assert_eq!(game.pots.players[0].balance,  60u128);
    assert_eq!(game.pots.players[1].balance,  70u128);
    assert_eq!(game.pots.players[2].balance, 140u128);
    assert_eq!(game.pots.players[3].balance,  80u128);
    assert_eq!(game.pots.list.len(), 1);
    assert_eq!(game.pots.list[0].balance.get("p0").unwrap(), &20);
    assert_eq!(game.pots.list[0].balance.get("p1").unwrap(), &20);
    assert_eq!(game.pots.list[0].balance.get("p2").unwrap(), &10);
    assert_eq!(game.pots.list[0].balance.get("p3").unwrap(), &20);
    assert_eq!(game.pots.closed.len(), 0);

    //call
    game.execute(Call {
        player_id: player2.id.to_string(),
    }).unwrap();
    assert_eq!(game.pots.players[0].balance,  60u128);
    assert_eq!(game.pots.players[1].balance,  70u128);
    assert_eq!(game.pots.players[2].balance, 130u128);
    assert_eq!(game.pots.players[3].balance,  80u128);
    assert_eq!(game.pots.list[0].balance.len(), 0);
    assert_eq!(game.pots.closed[0].balance.get("p0").unwrap(), &20);
    assert_eq!(game.pots.closed[0].balance.get("p1").unwrap(), &20);
    assert_eq!(game.pots.closed[0].balance.get("p2").unwrap(), &20);
    assert_eq!(game.pots.closed[0].balance.get("p3").unwrap(), &20);

    //flop
    game.execute(SetFlop {
        cards: vec![
            Card { suit: CardSuit::Club, number: CardNumber::Ace },
            Card { suit: CardSuit::Club, number: CardNumber::Two },
            Card { suit: CardSuit::Club, number: CardNumber::Three },
        ]
    }).unwrap();

    assert_eq!(game.status, Status::Flop);
    assert_eq!(game.community_cards.len(), 3);

    //bet
    game.execute(Raise {
        player_id: player0.id.to_string(),
        amount: 20
    }).unwrap();
    assert_eq!(game.pots.players[0].balance,  40u128);
    assert_eq!(game.pots.players[1].balance,  70u128);
    assert_eq!(game.pots.players[2].balance, 130u128);
    assert_eq!(game.pots.players[3].balance,  80u128);
    assert_eq!(game.pots.list.len(), 1);
    assert_eq!(game.pots.list[0].balance.get("p0").unwrap(), &20);

    assert_eq!(game.pots.closed[0].balance.get("p0").unwrap(), &20);
    assert_eq!(game.pots.closed[0].balance.get("p1").unwrap(), &20);
    assert_eq!(game.pots.closed[0].balance.get("p2").unwrap(), &20);
    assert_eq!(game.pots.closed[0].balance.get("p3").unwrap(), &20);

    //call
    game.execute(Call {
        player_id: player1.id.to_string(),
    }).unwrap();
    assert_eq!(game.pots.players[0].balance,  40u128);
    assert_eq!(game.pots.players[1].balance,  50u128);
    assert_eq!(game.pots.players[2].balance, 130u128);
    assert_eq!(game.pots.players[3].balance,  80u128);
    assert_eq!(game.pots.list.len(), 1);

    assert_eq!(game.pots.list[0].balance.get("p0").unwrap(), &20);
    assert_eq!(game.pots.list[0].balance.get("p1").unwrap(), &20);

    assert_eq!(game.pots.closed[0].balance.get("p0").unwrap(), &20);
    assert_eq!(game.pots.closed[0].balance.get("p1").unwrap(), &20);
    assert_eq!(game.pots.closed[0].balance.get("p2").unwrap(), &20);
    assert_eq!(game.pots.closed[0].balance.get("p3").unwrap(), &20);

    //call
    game.execute(Call {
        player_id: player2.id.to_string(),
    }).unwrap();
    assert_eq!(game.pots.players[0].balance,  40u128);
    assert_eq!(game.pots.players[1].balance,  50u128);
    assert_eq!(game.pots.players[2].balance, 110u128);
    assert_eq!(game.pots.players[3].balance,  80u128);
    assert_eq!(game.pots.list.len(), 1);

    assert_eq!(game.pots.list[0].balance.get("p0").unwrap(), &20);
    assert_eq!(game.pots.list[0].balance.get("p1").unwrap(), &20);
    assert_eq!(game.pots.list[0].balance.get("p2").unwrap(), &20);

    assert_eq!(game.pots.closed[0].balance.get("p0").unwrap(), &20);
    assert_eq!(game.pots.closed[0].balance.get("p1").unwrap(), &20);
    assert_eq!(game.pots.closed[0].balance.get("p2").unwrap(), &20);
    assert_eq!(game.pots.closed[0].balance.get("p3").unwrap(), &20);

    //raise
    game.execute(Raise {
        player_id: player3.id.to_string(),
        amount: 70,
    }).unwrap();
    assert_eq!(game.pots.players[0].balance,  40u128);
    assert_eq!(game.pots.players[1].balance,  50u128);
    assert_eq!(game.pots.players[2].balance, 110u128);
    assert_eq!(game.pots.players[3].balance,  10u128);
    assert_eq!(game.pots.list.len(), 1);

    assert_eq!(game.pots.list[0].balance.get("p0").unwrap(), &20);
    assert_eq!(game.pots.list[0].balance.get("p1").unwrap(), &20);
    assert_eq!(game.pots.list[0].balance.get("p2").unwrap(), &20);
    assert_eq!(game.pots.list[0].balance.get("p3").unwrap(), &70);

    assert_eq!(game.pots.closed[0].balance.get("p0").unwrap(), &20);
    assert_eq!(game.pots.closed[0].balance.get("p1").unwrap(), &20);
    assert_eq!(game.pots.closed[0].balance.get("p2").unwrap(), &20);
    assert_eq!(game.pots.closed[0].balance.get("p3").unwrap(), &20);

    //call, allin

    game.execute(Call {
        player_id: player0.id.to_string(),
    }).unwrap();
    assert_eq!(game.pots.players[0].balance,   0u128);
    assert_eq!(game.pots.players[1].balance,  50u128);
    assert_eq!(game.pots.players[2].balance, 110u128);
    assert_eq!(game.pots.players[3].balance,  10u128);

    assert_eq!(game.pots.list[0].balance.get("p0").unwrap(), &60);
    assert_eq!(game.pots.list[0].balance.get("p1").unwrap(), &20);
    assert_eq!(game.pots.list[0].balance.get("p2").unwrap(), &20);
    assert_eq!(game.pots.list[0].balance.get("p3").unwrap(), &60);

    assert_eq!(game.pots.list[1].balance.get("p3").unwrap(), &10);

    assert_eq!(game.pots.closed[0].balance.get("p0").unwrap(), &20);
    assert_eq!(game.pots.closed[0].balance.get("p1").unwrap(), &20);
    assert_eq!(game.pots.closed[0].balance.get("p2").unwrap(), &20);
    assert_eq!(game.pots.closed[0].balance.get("p3").unwrap(), &20);

    //call, allin
    game.execute(Call {
        player_id: player1.id.to_string(),
    }).unwrap();
    assert_eq!(game.pots.players[0].balance,   0u128);
    assert_eq!(game.pots.players[1].balance,   0u128);
    assert_eq!(game.pots.players[2].balance, 110u128);
    assert_eq!(game.pots.players[3].balance,  10u128);
    assert_eq!(game.pots.list.len(), 2);
    assert_eq!(game.pots.list[0].balance.get("p0").unwrap(), &60);
    assert_eq!(game.pots.list[0].balance.get("p1").unwrap(), &60);
    assert_eq!(game.pots.list[0].balance.get("p2").unwrap(), &20);
    assert_eq!(game.pots.list[0].balance.get("p3").unwrap(), &60);

    assert_eq!(game.pots.list[1].balance.get("p3").unwrap(), &10);
    assert_eq!(game.pots.list[1].balance.get("p1").unwrap(), &10);

    assert_eq!(game.pots.closed[0].balance.get("p0").unwrap(), &20);
    assert_eq!(game.pots.closed[0].balance.get("p1").unwrap(), &20);
    assert_eq!(game.pots.closed[0].balance.get("p2").unwrap(), &20);
    assert_eq!(game.pots.closed[0].balance.get("p3").unwrap(), &20);

    //call
    game.execute(Call {
        player_id: player2.id.to_string(),
    }).unwrap();
    assert_eq!(game.pots.players[0].balance,   0u128);
    assert_eq!(game.pots.players[1].balance,   0u128);
    assert_eq!(game.pots.players[2].balance,  60u128);
    assert_eq!(game.pots.players[3].balance,  10u128);
    assert_eq!(game.pots.list.len(), 1);

    // assert_eq!(game.pots.list, vec![]);
    assert_eq!(game.pots.closed[0].balance.get("p0").unwrap(), &20);
    assert_eq!(game.pots.closed[0].balance.get("p1").unwrap(), &20);
    assert_eq!(game.pots.closed[0].balance.get("p2").unwrap(), &20);
    assert_eq!(game.pots.closed[0].balance.get("p3").unwrap(), &20);

    assert_eq!(game.pots.closed[1].balance.get("p0").unwrap(), &60);
    assert_eq!(game.pots.closed[1].balance.get("p1").unwrap(), &60);
    assert_eq!(game.pots.closed[1].balance.get("p2").unwrap(), &60);
    assert_eq!(game.pots.closed[1].balance.get("p3").unwrap(), &60);

    assert_eq!(game.pots.closed[2].balance.get("p3").unwrap(), &10);
    assert_eq!(game.pots.closed[2].balance.get("p1").unwrap(), &10);
    assert_eq!(game.pots.closed[2].balance.get("p2").unwrap(), &10);

    //flop
    game.execute(SetTurn {
        card: Card { suit: CardSuit::Club, number: CardNumber::Four },
    }).unwrap();

    assert_eq!(game.status, Status::Turn);
    assert_eq!(game.community_cards.len(), 4);



    // assert_eq!(game.get_history(), History {
    //     player_cards: vec![],
    //     list: vec![],
    // });
}
//
// fn start_preflop_check() {
//     let cards = vec![
//         Card { suit: CardSuit::Club, number: CardNumber::Ace },
//         Card { suit: CardSuit::Club, number: CardNumber::Two },
//         Card { suit: CardSuit::Club, number: CardNumber::Three },
//         Card { suit: CardSuit::Club, number: CardNumber::Four },
//         Card { suit: CardSuit::Club, number: CardNumber::Five },
//     ];
//
//     let player0 = PlayerInitMsg {
//         id: "p0".to_string(),
//         balance: 80,
//         straddle: false,
//         cards: vec![
//             Card { suit: CardSuit::Spade, number: CardNumber::Queen },
//             Card { suit: CardSuit::Spade, number: CardNumber::King },
//         ]
//     };
//
//     let player1 = PlayerInitMsg {
//         id: "p1".to_string(),
//         balance: 90,
//         straddle: false,
//         cards: vec![
//             Card { suit: CardSuit::Spade, number: CardNumber::Ten },
//             Card { suit: CardSuit::Spade, number: CardNumber::Jack },
//         ]
//     };
//
//     let player2 = PlayerInitMsg {
//         id: "p2".to_string(),
//         balance: 150,
//         straddle: false,
//         cards: vec![
//             Card { suit: CardSuit::Spade, number: CardNumber::Three },
//             Card { suit: CardSuit::Spade, number: CardNumber::Four },
//         ]
//     };
//
//     let player3 = PlayerInitMsg {
//         id: "p3".to_string(),
//         balance: 100,
//         straddle: false,
//         cards: vec![
//             Card { suit: CardSuit::Heart, number: CardNumber::Five },
//             Card { suit: CardSuit::Heart, number: CardNumber::Six },
//         ]
//     };
//
//     let mut game = Game::new(5, 10, vec![player0.clone(), player1.clone(), player2.clone(), player3.clone()], cards).unwrap();
//
//     //small
//     game.small_blind().unwrap();
//     assert_eq!(game.pots.players[0].balance,  75u128);
//     assert_eq!(game.pots.players[1].balance,  90u128);
//     assert_eq!(game.pots.players[2].balance, 150u128);
//     assert_eq!(game.pots.players[3].balance, 100u128);
//     assert_eq!(game.pots.list.len(), 1);
//     assert_eq!(game.pots.list[0].balance.get("p0").unwrap(), &5);
//
//     assert_eq!(game.pots.closed.len(), 0);
//
//     //big
//     game.big_blind().unwrap();
//     assert_eq!(game.pots.players[0].balance,  75u128);
//     assert_eq!(game.pots.players[1].balance,  80u128);
//     assert_eq!(game.pots.players[2].balance, 150u128);
//     assert_eq!(game.pots.players[3].balance, 100u128);
//     assert_eq!(game.pots.list.len(), 1);
//     assert_eq!(game.pots.list[0].balance.get("p0").unwrap(), &5);
//     assert_eq!(game.pots.list[0].balance.get("p1").unwrap(), &10);
//
//     assert_eq!(game.pots.closed.len(), 0);
//
//     //call
//     game.call(&"p2".to_string()).unwrap();
//     assert_eq!(game.pots.players[0].balance,  75u128);
//     assert_eq!(game.pots.players[1].balance,  80u128);
//     assert_eq!(game.pots.players[2].balance, 140u128);
//     assert_eq!(game.pots.players[3].balance, 100u128);
//     assert_eq!(game.pots.list.len(), 1);
//     assert_eq!(game.pots.list[0].balance.get("p0").unwrap(), &5);
//     assert_eq!(game.pots.list[0].balance.get("p1").unwrap(), &10);
//     assert_eq!(game.pots.list[0].balance.get("p2").unwrap(), &10);
//
//     assert_eq!(game.pots.closed.len(), 0);
//
//     //call
//     game.call(&"p3".to_string()).unwrap();
//     assert_eq!(game.pots.players[0].balance,  75u128);
//     assert_eq!(game.pots.players[1].balance,  80u128);
//     assert_eq!(game.pots.players[2].balance, 140u128);
//     assert_eq!(game.pots.players[3].balance,  90u128);
//     assert_eq!(game.pots.list.len(), 1);
//     assert_eq!(game.pots.list[0].balance.get("p0").unwrap(), &5);
//     assert_eq!(game.pots.list[0].balance.get("p1").unwrap(), &10);
//     assert_eq!(game.pots.list[0].balance.get("p2").unwrap(), &10);
//     assert_eq!(game.pots.list[0].balance.get("p3").unwrap(), &10);
//
//     assert_eq!(game.pots.closed.len(), 0);
//
//     //call
//     game.call(&"p0".to_string()).unwrap();
//     assert_eq!(game.pots.players[0].balance,  70u128);
//     assert_eq!(game.pots.players[1].balance,  80u128);
//     assert_eq!(game.pots.players[2].balance, 140u128);
//     assert_eq!(game.pots.players[3].balance,  90u128);
//     assert_eq!(game.pots.list.len(), 1);
//     assert_eq!(game.pots.list[0].balance.get("p0").unwrap(), &10);
//     assert_eq!(game.pots.list[0].balance.get("p1").unwrap(), &10);
//     assert_eq!(game.pots.list[0].balance.get("p2").unwrap(), &10);
//     assert_eq!(game.pots.list[0].balance.get("p3").unwrap(), &10);
//
//     assert_eq!(game.pots.closed.len(), 0);
//
//     //check
//     game.check(&"p1".to_string()).unwrap();
//     assert_eq!(game.pots.players[0].balance,  70u128);
//     assert_eq!(game.pots.players[1].balance,  80u128);
//     assert_eq!(game.pots.players[2].balance, 140u128);
//     assert_eq!(game.pots.players[3].balance,  90u128);
//     assert_eq!(game.pots.list.len(), 1);
//     assert_eq!(game.pots.closed[0].balance.get("p0").unwrap(), &10);
//     assert_eq!(game.pots.closed[0].balance.get("p1").unwrap(), &10);
//     assert_eq!(game.pots.closed[0].balance.get("p2").unwrap(), &10);
//     assert_eq!(game.pots.closed[0].balance.get("p3").unwrap(), &10);
//
//     assert_eq!(game.status, Status::Flop);
//     assert_eq!(game.community_cards.len(), 3);
//
//     game.check(&"p0".to_string()).unwrap();
//     game.check(&"p1".to_string()).unwrap();
//     game.check(&"p2".to_string()).unwrap();
//     game.check(&"p3".to_string()).unwrap();
//
//     assert_eq!(game.status, Status::Turn);
//     assert_eq!(game.community_cards.len(), 4);
//
//     game.check(&"p0".to_string()).unwrap();
//     game.check(&"p1".to_string()).unwrap();
//     game.check(&"p2".to_string()).unwrap();
//     game.check(&"p3".to_string()).unwrap();
//
//     assert_eq!(game.status, Status::River);
//     assert_eq!(game.community_cards.len(), 5);
//
//     game.fold(&"p0".to_string()).unwrap();
//     game.fold(&"p1".to_string()).unwrap();
//     game.raise(&"p2".to_string(), 10).unwrap();
//     game.fold(&"p3".to_string()).unwrap();
//
//     assert_eq!(game.status, Status::End);
//     assert_eq!(game.community_cards.len(), 5);
//
//     // assert_eq!(game.pots.list.ba, vec![]);
//     // assert_eq!(game.pots.closed.len(), 0);
//     // assert_eq!(game.get_history(), &vec![]);
// }