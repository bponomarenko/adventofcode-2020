use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref BYR_RE: Regex = Regex::new(r"^19[2-9][0-9]|200[0-2]$").unwrap();
    static ref IYR_RE: Regex = Regex::new(r"^20(1[0-9]|20)$").unwrap();
    static ref EYR_RE: Regex = Regex::new(r"^20(2[0-9]|30)$").unwrap();
    static ref HGT_RE: Regex = Regex::new(r"^(59|6[0-9]|7[0-6])in|(1([5-8][0-9]|9[0-3]))cm$").unwrap();
    static ref HCL_RE: Regex = Regex::new(r"^#[a-z0-9]{6}$").unwrap();
    static ref ECL_RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    static ref PID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
}

pub fn run(input: String) -> u32 {
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
            let name = field.split_terminator(':').nth(0).unwrap();
            let value = field.split_terminator(':').nth(1).unwrap();
            if is_field_valid(name, value) {
                passport_fields.push(name);
            }
        }
    }

    if is_passport_valid(&passport_fields) {
        valid_passports += 1;
    }
    valid_passports
}

fn is_passport_valid(passport_fields: &Vec<&str>) -> bool {
    passport_fields.len() == 8 || (passport_fields.len() == 7 && !passport_fields.contains(&"cid"))
}

fn is_field_valid(name: &str, value: &str) -> bool {
    match name {
        "byr" => BYR_RE.is_match(value),
        "iyr" => IYR_RE.is_match(value),
        "eyr" => EYR_RE.is_match(value),
        "hgt" => HGT_RE.is_match(value),
        "hcl" => HCL_RE.is_match(value),
        "ecl" => ECL_RE.is_match(value),
        "pid" => PID_RE.is_match(value),
        "cid" => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn should_properly_count_valid_passports() {
        assert_eq!(
            run(&String::from(
                "
                ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
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
            )),
            2
        );

        assert_eq!(
            run(&String::from(
                "
                pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
                hcl:#623a2f

                eyr:2029 ecl:blu cid:129 byr:1989
                iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

                hcl:#888785
                hgt:164cm byr:2001 iyr:2015 cid:88
                pid:545766238 ecl:hzl
                eyr:2022

                iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
                ",
            )),
            4
        );
    }

    #[test]
    fn should_properly_count_invalid_passports() {
        assert_eq!(
            run(&String::from(
                "
                eyr:1972 cid:100
                hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

                iyr:2019
                hcl:#602927 eyr:1967 hgt:170cm
                ecl:grn pid:012533040 byr:1946

                hcl:dab227 iyr:2012
                ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

                hgt:59cm ecl:zzz
                eyr:2038 hcl:74454a iyr:2023
                pid:3556412378 byr:2007
                ",
            )),
            0
        );
    }
}
