use std::collections::HashMap;

fn contains_shiny_gold(rules: &HashMap<&str, Vec<(usize, &str)>>, colour: &str) -> bool {
    rules
        .get(colour)
        .unwrap()
        .iter()
        .find(|(_, c)| *c == "shiny gold" || contains_shiny_gold(rules, *c))
        .is_some()
}

fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();

    let mut rules: HashMap<&str, Vec<(usize, &str)>> = HashMap::new();

    for rule in input.split("\n").filter(|l| !l.is_empty()) {
        let mut parts: Vec<&str> = rule.trim_end_matches(".").split(" bags contain ").collect();
        assert!(parts.len() == 2);
        let colour = parts.remove(0);

        if parts[0] == "no other bags" {
            rules.insert(colour, Vec::default());
            continue;
        } else {
            let contents: Vec<(usize, &str)> = parts[0]
                .split(", ")
                .map(|rule| {
                    let rule: Vec<&str> = rule.splitn(2, ' ').collect();
                    assert!(rule.len() == 2);
                    let count: usize = rule[0].parse().unwrap();
                    let colour = rule[1].trim_end_matches(" bags").trim_end_matches(" bag");
                    (count, colour)
                })
                .collect();
            rules.insert(colour, contents);
        }
    }

    let count = rules
        .keys()
        .filter(|c| contains_shiny_gold(&rules, c))
        .count();
    println!("{}", count);
}
