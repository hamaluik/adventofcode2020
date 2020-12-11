fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let sum: usize = input
        .split("\n\n")
        .filter(|i| !i.is_empty())
        .filter(|l| l.split("\n").count() > 0)
        .map(|group| {
            let people: Vec<Vec<char>> = group
                .split("\n")
                .filter(|l| !l.is_empty())
                .map(|p| p.chars().collect::<Vec<char>>())
                .collect();
            let mut count = 0;
            assert!(people.len() > 0);
            for q in people[0].iter() {
                let mut c = 1;
                for p in people.iter().skip(1) {
                    if p.iter().find(|&a| a == q).is_none() {
                        c = 0;
                        break;
                    }
                }
                count += c;
            }
            count
        })
        .sum();
    println!("{}", sum);
}
