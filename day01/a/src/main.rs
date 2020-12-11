fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let input: Vec<usize> = input
        .split("\n")
        .filter_map(|s| {
            if s.is_empty() {
                None
            } else {
                Some(s.parse().unwrap())
            }
        })
        .collect();

    for a in 0..input.len() {
        for b in (a + 1)..input.len() {
            if input[a] + input[b] == 2020 {
                println!("{}", input[a] * input[b]);
                return;
            }
        }
    }

    println!("?");
}
