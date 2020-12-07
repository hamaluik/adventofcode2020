use std::collections::HashMap;

fn main() {
    let data = std::fs::read_to_string("../input.txt").unwrap();
    let count = data
        .split("\n\n")
        .filter(|passport| {
            let mut data: HashMap<&str, &str> = HashMap::with_capacity(8);
            for field in passport.split_whitespace() {
                let parts: Vec<&str> = field.splitn(2, ":").collect();
                assert!(parts.len() == 2);
                data.insert(parts[0], parts[1]);
            }
            let valid = data.len() == 8
                || (data.len() == 7 && data.keys().find(|&d| d == &"cid").is_none());
            valid
        })
        .count();
    println!("{}", count);
}
