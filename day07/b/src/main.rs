use std::collections::HashMap;

fn contained_bags(rules: &HashMap<&str, Vec<(usize, &str)>>, colour: &str) -> usize {
    rules
        .get(colour)
        .unwrap()
        .iter()
        .map(|(cnt, colour)| (cnt * contained_bags(rules, colour)) + cnt)
        .sum()
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

    let count = contained_bags(&rules, "shiny gold");
    println!("{}", count);
}
