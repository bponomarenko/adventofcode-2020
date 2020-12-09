use std::collections::HashSet;

pub fn run(input: String) -> isize {
    let sequence: Vec<(&str, i32)> = input
        .split_terminator('\n')
        .map(|s| {
            let parsed: Vec<&str> = s.split_whitespace().collect();
            (parsed[0], parsed[1].parse::<i32>().unwrap())
        })
        .collect();

    let mut pos = 0;
    while pos < sequence.len() {
        match find_final_acc(&sequence, pos as isize) {
            Some(acc) => return acc,
            None => {
                pos += 1;
            }
        }
    }
    unreachable!();
}

fn find_final_acc(sequence: &Vec<(&str, i32)>, inverted_instr_pos: isize) -> Option<isize> {
    let mut visited = HashSet::new();
    let mut acc: isize = 0;
    let mut pos: isize = 0;

    while (pos as usize) < sequence.len() {
        if visited.contains(&pos) {
            return Some(acc);
        } else {
            visited.insert(pos);
        }

        let (instr, value) = sequence[pos as usize];
        let check_instr = if pos == inverted_instr_pos {
            match instr {
                "nop" => "jmp",
                "jmp" => "nop",
                _ => instr,
            }
        } else {
            instr
        };

        match check_instr {
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
    None
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
