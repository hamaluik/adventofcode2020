fn main() {
    let data = std::fs::read_to_string("../input.txt").unwrap();
    let count = data
        .split("\n\n")
        .filter(|passport| {
            let mut valid_count = 0;
            let mut has_cid = false;
            for field in passport.split_whitespace() {
                let mut parts: Vec<&str> = field.splitn(2, ":").collect();
                assert!(parts.len() == 2);
                let id = parts.remove(0);
                let value = parts.remove(0);

                if match id {
                    "byr" => match value.parse::<usize>() {
                        Ok(y) => y >= 1920 && y <= 2002,
                        Err(_) => false,
                    },
                    "iyr" => match value.parse::<usize>() {
                        Ok(y) => y >= 2010 && y <= 2020,
                        Err(_) => false,
                    },
                    "eyr" => match value.parse::<usize>() {
                        Ok(y) => y >= 2020 && y <= 2030,
                        Err(_) => false,
                    },
                    "hgt" => {
                        let units: String = value.chars().rev().take(2).collect();
                        let value: String = value.chars().take(value.len() - 2).collect();
                        match value.parse::<usize>() {
                            Ok(value) => match units.as_str() {
                                "mc" => value >= 150 && value <= 193,
                                "ni" => value >= 59 && value <= 76,
                                _ => false,
                            },
                            Err(_) => false,
                        }
                    }
                    "hcl" => {
                        value.chars().nth(0).unwrap() == '#'
                            && value.chars().skip(1).all(|c| c.is_digit(16))
                    }
                    "ecl" => match value {
                        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                        _ => false,
                    },
                    "pid" => value.len() == 9 && value.chars().all(|c| c.is_digit(10)),
                    "cid" => {
                        has_cid = true;
                        true
                    }
                    _ => false,
                } {
                    valid_count += 1;
                }
            }
            valid_count == 8 || (valid_count == 7 && !has_cid)
        })
        .count();
    println!("{}", count);
}
