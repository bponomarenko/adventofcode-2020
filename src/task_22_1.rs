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

pub fn run(input: String) -> usize {
    let (mut player1, mut player2) = parse_input(&input);

    while !player1.is_empty() && !player2.is_empty() {
        let card1 = player1.remove(0);
        let card2 = player2.remove(0);

        if card2 > card1 {
            player2.extend(&[card2, card1]);
        } else {
            player1.extend(&[card1, card2]);
        }
    }

    let winner = if player1.is_empty() { player2 } else { player1 };
    winner.iter().rev().enumerate().fold(0, |acc, (i, card)| acc + (*card as usize) * (i + 1))
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn should_run_correctly() {
        let input = String::from("Player 1:\n9\n2\n6\n3\n1\n\nPlayer 2:\n5\n8\n4\n7\n10");
        assert_eq!(run(input), 306);
    }
}
