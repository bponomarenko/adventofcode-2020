use lazy_static::lazy_static;
use regex::{Captures, Regex};

lazy_static! {
    static ref SUM_RE: Regex = Regex::new(r"(?P<arg1>\d+) \+ (?P<arg2>\d+)").unwrap();
    static ref GROUP_RE: Regex = Regex::new(r"\(([^()]+)\)").unwrap();
}

fn parse_arg(caps: &Captures, name: &str) -> u64 {
    caps.name(name)
        .unwrap()
        .as_str()
        .parse::<u64>()
        .expect("Expression argument should be a valid number")
}

fn simple_eval(expr: &str) -> u64 {
    let mut final_expr = expr.to_string();
    // First evaluate all addition operations (as they have higher priority)
    loop {
        match SUM_RE.captures(&final_expr.clone()) {
            Some(caps) => {
                let expr_res = parse_arg(&caps, "arg1") + parse_arg(&caps, "arg2");
                final_expr.replace_range(caps.get(0).unwrap().range(), &expr_res.to_string());
            }
            _ => break,
        }
    }
    // Since there are only additions and multiplications, only the latter is left
    final_expr
        .split_terminator(" * ")
        .map(|arg| arg.parse::<u64>().expect("Argument should be an integer"))
        .product()
}

fn eval(expr: &str) -> u64 {
    let mut final_expr = expr.to_string();
    // First evaluate all nested groups
    loop {
        match GROUP_RE.captures(&final_expr.clone()) {
            Some(caps) => {
                let group_result = simple_eval(caps.get(1).unwrap().as_str());
                // Replace grouped expression with group evaluation result
                final_expr.replace_range(caps.get(0).unwrap().range(), &group_result.to_string());
            }
            _ => break,
        }
    }
    // And then evaluate final ungrouped expression
    simple_eval(&final_expr)
}

pub fn run(input: String) -> u64 {
    input.split_terminator('\n').map(|expr| eval(expr)).sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn should_run_correctly1() {
        let input = String::from("1 + 2 * 3 + 4 * 5 + 6");
        assert_eq!(run(input), 231);
    }

    #[test]
    fn should_run_correctly2() {
        let input = String::from("1 + (2 * 3) + (4 * (5 + 6))");
        assert_eq!(run(input), 51);
    }

    #[test]
    fn should_run_correctly3() {
        let input = String::from("2 * 3 + (4 * 5)");
        assert_eq!(run(input), 46);
    }

    #[test]
    fn should_run_correctly4() {
        let input = String::from("5 + (8 * 3 + 9 + 3 * 4 * 3)");
        assert_eq!(run(input), 1445);
    }

    #[test]
    fn should_run_correctly5() {
        let input = String::from("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))");
        assert_eq!(run(input), 669060);
    }

    #[test]
    fn should_run_correctly6() {
        let input = String::from("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");
        assert_eq!(run(input), 23340);
    }
}
