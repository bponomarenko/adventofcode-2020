use std::collections::HashSet;

fn deserialize_card_hand(cards: &str) -> Vec<u8> {
    cards
        .split_terminator("\n")
        .skip(1)
        .map(|card| card.parse::<u8>().expect("Card is invalid"))
        .collect()
}

fn parse_input(input: &String) -> (Vec<u8>, Vec<u8>) {
    let players_cards = input.split_terminator("\n\n").collect::<Vec<&str>>();
    match players_cards.as_slice() {
        [p1_cards, p2_cards, ..] => (deserialize_card_hand(p1_cards), deserialize_card_hand(p2_cards)),
        _ => panic!("Input is invlaid"),
    }
}

enum Winner {
    Player1(Vec<u8>),
    Player2(Vec<u8>),
}

fn cards_to_string(cards: &Vec<u8>) -> String {
    cards.iter().map(|card| card.to_string()).collect::<Vec<String>>().join(",")
}

fn play_game(mut player1: Vec<u8>, mut player2: Vec<u8>) -> Winner {
    let mut rounds_history = HashSet::new();

    while !player1.is_empty() && !player2.is_empty() {
        let round_setup = format!("{}-{}", cards_to_string(&player1), cards_to_string(&player2));
        if rounds_history.contains(&round_setup) {
            // If such setup have been before – Player1 wins immediately
            return Winner::Player1(player1);
        } else {
            rounds_history.insert(round_setup);
        }

        let card1 = player1.remove(0);
        let card2 = player2.remove(0);

        if card1 as usize <= player1.len() && card2 as usize <= player2.len() {
            // Recursive combat!
            match play_game(player1[..card1 as usize].to_vec(), player2[..card2 as usize].to_vec()) {
                Winner::Player1(_) => player1.extend(&[card1, card2]),
                Winner::Player2(_) => player2.extend(&[card2, card1]),
            }
        } else {
            if card2 > card1 {
                player2.extend(&[card2, card1]);
            } else {
                player1.extend(&[card1, card2]);
            }
        }
    }

    if player1.is_empty() {
        Winner::Player2(player2)
    } else {
        Winner::Player1(player1)
    }
}

pub fn run(input: String) -> usize {
    let (player1, player2) = parse_input(&input);

    match play_game(player1, player2) {
        Winner::Player1(winner) | Winner::Player2(winner) => {
            winner.iter().rev().enumerate().fold(0, |acc, (i, card)| acc + (*card as usize) * (i + 1))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn should_run_correctly() {
        let input = String::from("Player 1:\n9\n2\n6\n3\n1\n\nPlayer 2:\n5\n8\n4\n7\n10");
        assert_eq!(run(input), 291);
    }
}
