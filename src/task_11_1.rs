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
                Seat::Occupied if count_occupied_adjacent_seats(&prev_seats, &r, &s) >= 4 => {
                    *seat = Seat::Free;
                    modified = true;
                }
                Seat::Free if count_occupied_adjacent_seats(&prev_seats, &r, &s) == 0 => {
                    *seat = Seat::Occupied;
                    modified = true;
                }
                _ => (),
            };
        }
    }
    modified
}

fn count_occupied_adjacent_seats(seats: &Seats, r: &usize, s: &usize) -> u8 {
    let row_min = if *r == 0 { 0 } else { r - 1 };
    let seat_min = if *s == 0 { 0 } else { s - 1 };
    let mut count = 0;

    for row in row_min..=r + 1 {
        if row < seats.len() {
            for seat in seat_min..=s + 1 {
                let seats_row = &seats[row];
                if seat < seats_row.len() && (row != *r || seat != *s) && seats_row[seat] == Seat::Occupied {
                    count += 1;
                }
            }
        }
    }
    count
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

pub fn run(input: String) -> usize {
    let mut seats = parse_input(input);
    while run_round(&mut seats) {}
    count_occupied_seats(&seats) as usize
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn should_run_correctly() {
        let input = String::from("L.LL.LL.LL LLLLLLL.LL L.L.L..L.. LLLL.LL.LL L.LL.LL.LL L.LLLLL.LL ..L.L..... LLLLLLLLLL L.LLLLLL.L L.LLLLL.LL");
        assert_eq!(run(input), 37);
    }
}
