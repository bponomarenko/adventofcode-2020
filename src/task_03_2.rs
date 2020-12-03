pub fn run(input: &String) {
    let lines: Vec<&str> = input.split_terminator('\n').collect();
    let routes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut res = 1;

    for route in routes.iter() {
        res *= find_trees(route.0, route.1, &lines);
    }

    println!("Result: {}", res);
}

fn find_trees(dx: u16, dy: u16, lines: &Vec<&str>) -> usize {
    let lines_len = lines.len() as u16;
    let line_len = lines[0].len() as u16;
    let mut count = 0;
    let mut x = dx;
    let mut y = dy;

    loop {
        if lines[y as usize]
            .chars()
            .nth(if x >= line_len { x % line_len } else { x } as usize)
            .unwrap()
            .eq(&'#')
        {
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
    use super::*;

    const INPUT: &str = "..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#";

    #[test]
    fn should_find_all_trees() {
        assert_eq!(find_trees(1, 1, &INPUT.split_terminator('\n').collect()), 2);
        assert_eq!(find_trees(3, 1, &INPUT.split_terminator('\n').collect()), 7);
        assert_eq!(find_trees(5, 1, &INPUT.split_terminator('\n').collect()), 3);
        assert_eq!(find_trees(7, 1, &INPUT.split_terminator('\n').collect()), 4);
        assert_eq!(find_trees(1, 2, &INPUT.split_terminator('\n').collect()), 2);
    }
}
