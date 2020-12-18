use lazy_static::lazy_static;
use regex::{Captures, Regex};

lazy_static! {
    static ref SIMPLE_EXPR_RE: Regex = Regex::new(r"(?P<arg1>\d+) (?P<operation>[*+]) (?P<arg2>\d+)").unwrap();
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
    // Evaluate expression by arguments pair
    loop {
        match SIMPLE_EXPR_RE.captures(&final_expr.clone()) {
            Some(caps) => {
                let arg1 = parse_arg(&caps, "arg1");
                let arg2 = parse_arg(&caps, "arg2");
                // There are only sum and multiply operations
                let expr_res = if caps.name("operation").unwrap().as_str() == "*" {
                    arg1 * arg2
                } else {
                    arg1 + arg2
                };
                final_expr.replace_range(caps.get(0).unwrap().range(), &expr_res.to_string());
            }
            _ => break,
        }
    }
    final_expr.parse::<u64>().expect("Simple expression cannot be evaluated")
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
        assert_eq!(run(input), 71);
    }

    #[test]
    fn should_run_correctly2() {
        let input = String::from("1 + (2 * 3) + (4 * (5 + 6))");
        assert_eq!(run(input), 51);
    }

    #[test]
    fn should_run_correctly3() {
        let input = String::from("2 * 3 + (4 * 5)");
        assert_eq!(run(input), 26);
    }

    #[test]
    fn should_run_correctly4() {
        let input = String::from("5 + (8 * 3 + 9 + 3 * 4 * 3)");
        assert_eq!(run(input), 437);
    }

    #[test]
    fn should_run_correctly5() {
        let input = String::from("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))");
        assert_eq!(run(input), 12240);
    }

    #[test]
    fn should_run_correctly6() {
        let input = String::from("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");
        assert_eq!(run(input), 13632);
    }
}
