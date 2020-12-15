fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(congruences: Vec<(i64, i64)>) -> Option<i64> {
    let prod = congruences.iter().map(|(m, _)| m).product::<i64>();
    let mut sum = 0;

    for (modulus, residue) in congruences.iter() {
        let p = prod / modulus;
        sum += residue * mod_inv(p, *modulus)? * p
    }
    Some(sum % prod)
}

pub fn run(input: String) -> i64 {
    let congruences: Vec<(i64, i64)> = input
        .split_terminator(',')
        .enumerate()
        .filter(|(_, value)| *value != "x")
        .map(|(i, value)| {
            let modulus = value.trim().parse::<i64>().expect("Modulus should be a valid integer");
            (modulus, modulus - i as i64)
        })
        .collect();

    chinese_remainder(congruences).unwrap()
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn should_run_correctly() {
        assert_eq!(run(String::from("7,13,x,x,59,x,31,19")), 1068781);
        assert_eq!(run(String::from("17,x,13,19")), 3417);
        assert_eq!(run(String::from("67,7,59,61")), 754018);
        assert_eq!(run(String::from("67,x,7,59,61")), 779210);
        assert_eq!(run(String::from("67,7,x,59,61")), 1261476);
        assert_eq!(run(String::from("1789,37,47,1889")), 1202161486);
    }
}
