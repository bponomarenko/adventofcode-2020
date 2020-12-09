use std::collections::HashSet;

pub fn run(input: String) -> isize {
    let sequence: Vec<(&str, i32)> = input
        .split_terminator('\n')
        .map(|s| {
            let parsed: Vec<&str> = s.split_whitespace().collect();
            (parsed[0], parsed[1].parse::<i32>().unwrap())
        })
        .collect();

    let mut visited = HashSet::new();
    let mut acc: isize = 0;
    let mut pos: isize = 0;

    while (pos as usize) < sequence.len() {
        if visited.contains(&pos) {
            return acc;
        } else {
            visited.insert(pos);
        }

        let (instr, value) = sequence[pos as usize];
        match instr {
            "acc" => {
                acc += value as isize;
                pos += 1;
            }
            "jmp" => {
                pos += value as isize;
            }
            _ => pos += 1,
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn should_run_correctly() {
        let input = String::from("nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6");
        assert_eq!(run(input), 5);
    }
}
