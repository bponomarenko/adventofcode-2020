type Coord = (i32, i32);

fn move_ship(direction: &char, value: i32, pos: Coord) -> Coord {
    match direction {
        'E' => (pos.0 + value, pos.1),
        'W' => (pos.0 - value, pos.1),
        'N' => (pos.0, pos.1 + value),
        'S' => (pos.0, pos.1 - value),
        _ => pos,
    }
}

pub fn run(input: String) -> i32 {
    let directions = ['E', 'S', 'W', 'N'];
    let mut ship: Coord = (0, 0);
    let mut ship_direction = &'E';

    input.split_whitespace().for_each(|str| {
        let instr = str.chars().nth(0).unwrap();
        let value = str[1..].parse::<i32>().unwrap();

        match instr {
            'F' => ship = move_ship(ship_direction, value, ship),
            'R' => {
                ship_direction = directions
                    .iter()
                    .cycle()
                    .skip_while(|d| *d != ship_direction)
                    .skip((value / 90) as usize)
                    .next()
                    .unwrap()
            }
            'L' => {
                ship_direction = directions
                    .iter()
                    .rev()
                    .cycle()
                    .skip_while(|d| *d != ship_direction)
                    .skip((value / 90) as usize)
                    .next()
                    .unwrap()
            }
            _ => ship = move_ship(&instr, value, ship),
        }
    });

    ship.0.abs() + ship.1.abs()
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn should_run_correctly() {
        let input = String::from("F10 N3 F7 R90 F11");
        assert_eq!(run(input), 25);
    }
}
