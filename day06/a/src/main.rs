fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let sum: usize = input
        .split("\n\n")
        .filter(|i| !i.is_empty())
        .map(|group| {
            let mut set = std::collections::HashSet::new();
            for person in group.split("\n") {
                for answer in person.chars() {
                    set.insert(answer);
                }
            }
            set.len()
        })
        .sum();
    println!("{}", sum);
}
