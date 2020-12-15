fn find_bus_id(input: String, timestamp: u32) -> u32 {
    let (id, minutes) = input
        .split_terminator(',')
        .filter(|id| *id != "x")
        .map(|id| {
            let id = id.trim().parse::<u32>().unwrap();
            (id, id - timestamp % id)
        })
        .min_by(|(_, i1), (_, i2)| i1.cmp(&i2))
        .unwrap();
    id * minutes
}

pub fn run(input: String) -> u32 {
    find_bus_id(input, 1002394)
}

#[cfg(test)]
mod tests {
    use super::find_bus_id;

    #[test]
    fn should_run_correctly() {
        assert_eq!(find_bus_id(String::from("7,13,x,x,59,x,31,19"), 939), 295);
    }
}
