pub fn run(input: &String) {
    let entries: Vec<u32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Input should consist of numbers"))
        .collect();

    println!("Result: {}", find_matching_entries(&entries));
}

fn find_matching_entries(entries: &Vec<u32>) -> u32 {
    for (i, entry) in entries.iter().enumerate() {
        for sub_entry in entries[(i + 1)..entries.len()].iter() {
            if entry + sub_entry == 2020 {
                return entry * sub_entry;
            }
        }
    }
    return 0;
}
