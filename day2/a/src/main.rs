fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let count = input
        .split("\n")
        .filter(|s| {
            if s.is_empty() {
                return false;
            }

            let mut parts: Vec<&str> = s.split(": ").collect();
            if parts.len() != 2 {
                return false;
            }

            let policy = parts.remove(0);
            let password = parts.remove(0);

            let mut parts: Vec<&str> = policy.split(' ').collect();
            if parts.len() != 2 {
                return false;
            }
            let mut range: Vec<&str> = parts.remove(0).split('-').collect();
            let c: char = parts.remove(0).chars().next().unwrap();

            let low = range.remove(0);
            let high = range.remove(0);
            let low: usize = low.parse().unwrap();
            let high: usize = high.parse().unwrap();

            let count = password.chars().filter(|&ch| ch == c).count();

            count >= low && count <= high
        })
        .count();
    println!("{}", count);
}
