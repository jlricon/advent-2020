fn main() {
    let input = include_str!("../../input/day4.txt");

    let passports: Vec<&str> = input.split("\n\n").collect();

    let valid_passports: Vec<bool> = passports
        .iter()
        .map(|passport| {
            ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
                .iter()
                .map(|have| passport.contains(have))
                .all(|v| v)
        })
        .collect();
    let part1: i32 = valid_passports.iter().map(|n| *n as i32).sum();
    fn validate_num(passport: &str, key: &str, min: i32, max: i32) -> bool {
        passport[passport.find(key).unwrap() + 4..]
            .chars()
            .take(4)
            .collect::<String>()
            .parse::<i32>()
            .map_or(false, |val| val >= min && val <= max)
    };
    let part2: i32 = passports
        .iter()
        .zip(valid_passports)
        .filter(|(_, is_valid)| *is_valid)
        .map(|(passport, _)| {
            let valid_byr = validate_num(passport, "byr", 1920, 2002);
            let valid_iyr = validate_num(passport, "iyr", 2010, 2020);
            let valid_eyr = validate_num(passport, "eyr", 2020, 2030);

            let pid: String = passport[passport.find("pid").unwrap() + 4..]
                .chars()
                .take_while(|n| n.is_numeric())
                .collect();
            let valid_pid = pid.len() == 9 && pid.chars().all(|c| c.is_numeric());

            let ecl = passport[passport.find("ecl").unwrap() + 4..]
                .chars()
                .take(3)
                .collect::<String>();

            let valid_ecl = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                .iter()
                .map(|eye_color| ecl == **eye_color)
                .any(|v| v);

            let hcl = passport[passport.find("hcl").unwrap() + 4..]
                .chars()
                .take(7)
                .collect::<String>();

            let hcl_has_pound = hcl.chars().next().unwrap() == '#';

            let hcl_is_ascii = hcl
                .chars()
                .skip(1)
                .take(6)
                .filter(|&c| (c >= 'a' && c <= 'f') || (c >= '0' && c <= '9'))
                .count()
                == 6;

            let valid_hcl = hcl_has_pound && hcl_is_ascii;
            let hgt: String = passport[passport.find("hgt").unwrap() + 4..]
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric())
                .collect();

            let num_val = hgt
                .chars()
                .take_while(|c| c.is_numeric())
                .collect::<String>()
                .parse::<i32>()
                .unwrap();

            let height_units: String = hgt.chars().skip_while(|c| c.is_numeric()).collect();

            let valid_hgt = match (num_val, height_units.as_ref()) {
                (num, "cm") if num >= 150 && num <= 193 => true,
                (num, "in") if num >= 59 && num <= 76 => true,
                _ => false,
            };

            if valid_byr
                && valid_iyr
                && valid_eyr
                && valid_pid
                && valid_ecl
                && valid_hcl
                && valid_hgt
            {
                1
            } else {
                0
            }
        })
        .sum();

    println!("{:?}", part1);
    println!("{:?}", part2);
}
