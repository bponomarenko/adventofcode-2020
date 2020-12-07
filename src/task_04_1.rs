pub fn run(input: String) {
    println!("Valid passports: {}", count_valid_passports(input));
}

fn is_passport_valid(passport_fields: &Vec<&str>) -> bool {
    passport_fields.len() == 8 || (passport_fields.len() == 7 && !passport_fields.contains(&"cid"))
}

fn count_valid_passports(input: &String) -> u32 {
    let mut valid_passports = 0;
    let mut passport_fields: Vec<&str> = Vec::new();

    for line in input.split_terminator('\n') {
        if line.trim().is_empty() {
            // reached end of the passport definition
            if is_passport_valid(&passport_fields) {
                valid_passports += 1;
            }
            passport_fields.clear();
            continue;
        }

        for field in line.split_whitespace() {
            passport_fields.push(field.split_terminator(':').nth(0).unwrap());
        }
    }

    if is_passport_valid(&passport_fields) {
        valid_passports += 1;
    }
    return valid_passports;
}

#[cfg(test)]
mod tests {
    use super::count_valid_passports;

    #[test]
    fn should_properly_count_valid_passports() {
        let input = String::from(
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm

        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929

        hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm

        hcl:#cfa07d eyr:2025 pid:166559648
        iyr:2011 ecl:brn hgt:59in
        ",
        );
        assert_eq!(count_valid_passports(&input), 2);
    }
}
