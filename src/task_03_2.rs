pub fn run(input: String) -> usize {
    let lines: Vec<&str> = input.split_terminator('\n').collect();
    let routes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut res = 1;

    for route in routes.iter() {
        res *= find_trees(route.0, route.1, &lines);
    }
    res
}

fn find_trees(dx: u16, dy: u16, lines: &Vec<&str>) -> usize {
    let lines_len = lines.len() as u16;
    let line_len = lines[0].len() as u16;
    let mut count = 0;
    let mut x = dx;
    let mut y = dy;

    loop {
        if lines[y as usize].chars().nth((x % line_len) as usize).unwrap().eq(&'#') {
            count += 1;
        }

        x += dx;
        y += dy;
        if y >= lines_len {
            break;
        }
    }

    return count;
}

#[cfg(test)]
mod tests {
    use super::find_trees;

    const INPUT: &str = "..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#";

    #[test]
    fn should_find_all_trees() {
        let lines = &INPUT.split_terminator('\n').collect();
        assert_eq!(find_trees(1, 1, lines), 2);
        assert_eq!(find_trees(3, 1, lines), 7);
        assert_eq!(find_trees(5, 1, lines), 3);
        assert_eq!(find_trees(7, 1, lines), 4);
        assert_eq!(find_trees(1, 2, lines), 2);
    }
}
