type Coord = (i32, i32);

fn move_waypoint(direction: &char, value: i32, pos: Coord) -> Coord {
    match direction {
        'E' => (pos.0 + value, pos.1),
        'W' => (pos.0 - value, pos.1),
        'N' => (pos.0, pos.1 + value),
        'S' => (pos.0, pos.1 - value),
        _ => pos,
    }
}

fn rotate_waypoint(direction: &char, degrees: i32, pos: Coord) -> Coord {
    match degrees {
        90 => {
            if *direction == 'R' {
                (pos.1, -pos.0)
            } else {
                (-pos.1, pos.0)
            }
        }
        180 => (-pos.0, -pos.1),
        270 => {
            if *direction == 'R' {
                (-pos.1, pos.0)
            } else {
                (pos.1, -pos.0)
            }
        }
        _ => pos,
    }
}

pub fn run(input: String) -> i32 {
    let mut ship: Coord = (0, 0);
    let mut waypoint: Coord = (10, 1);

    input.split_whitespace().for_each(|str| {
        let instr = str.chars().nth(0).unwrap();
        let value = str[1..].parse::<i32>().unwrap();

        match instr {
            'F' => ship = (ship.0 + value * waypoint.0, ship.1 + value * waypoint.1),
            'R' | 'L' => waypoint = rotate_waypoint(&instr, value, waypoint),
            _ => waypoint = move_waypoint(&instr, value, waypoint),
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
        assert_eq!(run(input), 286);
    }
}
