pub fn run(input: String) {
    let lines: Vec<&str> = input.split_terminator('\n').collect();
    println!("Result: {}", find_trees(3, 1, &lines));
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
        assert_eq!(find_trees(3, 1, &INPUT.split_terminator('\n').collect()), 7);
    }
}
