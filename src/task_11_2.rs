#[derive(Clone, PartialEq)]
enum Seat {
    NoSeat,
    Free,
    Occupied,
}

type Seats = Vec<Vec<Seat>>;

fn parse_input(input: String) -> Seats {
    input
        .split_whitespace()
        .map(|row| {
            row.chars()
                .map(|s| match s {
                    '#' => Seat::Occupied,
                    'L' => Seat::Free,
                    _ => Seat::NoSeat,
                })
                .collect()
        })
        .collect()
}

fn run_round(seats: &mut Seats) -> bool {
    let prev_seats = seats.clone();
    let mut modified = false;

    for (r, row) in seats.iter_mut().enumerate() {
        for (s, seat) in row.iter_mut().enumerate() {
            match seat {
                Seat::Occupied if count_occupied_visible_seats(&prev_seats, &r, &s) >= 5 => {
                    *seat = Seat::Free;
                    modified = true;
                }
                Seat::Free if count_occupied_visible_seats(&prev_seats, &r, &s) == 0 => {
                    *seat = Seat::Occupied;
                    modified = true;
                }
                _ => (),
            };
        }
    }
    modified
}

fn count_occupied_visible_seats(seats: &Seats, r: &usize, s: &usize) -> u8 {
    count_occupied_direction_seats(seats, r, s, 0, 1)
        + count_occupied_direction_seats(seats, r, s, -1, 1)
        + count_occupied_direction_seats(seats, r, s, -1, 0)
        + count_occupied_direction_seats(seats, r, s, -1, -1)
        + count_occupied_direction_seats(seats, r, s, 0, -1)
        + count_occupied_direction_seats(seats, r, s, 1, -1)
        + count_occupied_direction_seats(seats, r, s, 1, 0)
        + count_occupied_direction_seats(seats, r, s, 1, 1)
}

fn count_occupied_direction_seats(seats: &Seats, r: &usize, s: &usize, dr: i32, ds: i32) -> u8 {
    let mut next_r = *r as i32;
    let mut next_s = *s as i32;

    loop {
        next_r += dr;
        next_s += ds;

        if next_r < 0 || next_r >= seats.len() as i32 || next_s < 0 || next_s >= seats[0].len() as i32 {
            break;
        }
        match seats[next_r as usize][next_s as usize] {
            Seat::Occupied => return 1,
            Seat::Free => return 0,
            _ => (),
        }
    }
    0
}

fn count_occupied_seats(seats: &Seats) -> u32 {
    let mut count = 0;
    for row in seats {
        for seat in row {
            if *seat == Seat::Occupied {
                count += 1;
            }
        }
    }
    count
}

pub fn run(input: String) -> u32 {
    let mut seats = parse_input(input);
    while run_round(&mut seats) {}
    count_occupied_seats(&seats)
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn should_run_correctly() {
        let input = String::from("L.LL.LL.LL LLLLLLL.LL L.L.L..L.. LLLL.LL.LL L.LL.LL.LL L.LLLLL.LL ..L.L..... LLLLLLLLLL L.LLLLLL.L L.LLLLL.LL");
        assert_eq!(run(input), 26);
    }
}
