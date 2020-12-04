const TO_VALIDATE: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
fn main() {
    let input = include_str!("../../input/day4.txt");

    let passports: Vec<&str> = input.split("\n\n").collect();

    let valid_passports: Vec<bool> = passports
        .iter()
        .map(|passport| {
            TO_VALIDATE
                .iter()
                .map(|have| passport.contains(have))
                .all(|v| v == true)
        })
        .collect();
    let part1: i32 = valid_passports.iter().map(|n| *n as i32).sum();
    let part2: i32 = passports
        .iter()
        .zip(valid_passports)
        .filter(|(_, is_valid)| *is_valid)
        .map(|(passport, _)| {
            let valid_byr = passport[passport.find("byr").unwrap() + 4..]
                .chars()
                .take(4)
                .collect::<String>()
                .parse::<i32>()
                .map_or(false, |val| val >= 1920 && val <= 2002);

            let valid_iyr = passport[passport.find("iyr").unwrap() + 4..]
                .chars()
                .take(4)
                .collect::<String>()
                .parse::<i32>()
                .map_or(false, |val| val >= 2010 && val <= 2020);

            let valid_eyr = passport[passport.find("eyr").unwrap() + 4..]
                .chars()
                .take(4)
                .collect::<String>()
                .parse::<i32>()
                .map_or(false, |val| val >= 2020 && val <= 2030);

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
                .parse::<i32>();

            let height_units: String = hgt.chars().skip_while(|c| c.is_numeric()).collect();

            let valid_hgt = match num_val {
                Ok(hgt_num) => {
                    if height_units == "cm" {
                        hgt_num >= 150 && hgt_num <= 193
                    } else if height_units == "in" {
                        hgt_num >= 59 && hgt_num <= 76
                    } else {
                        false
                    }
                }
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
