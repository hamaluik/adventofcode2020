fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let lines: Vec<usize> = input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect();

    let mut window: std::collections::VecDeque<usize> =
        std::collections::VecDeque::with_capacity(25);
    for (i, &num) in lines.iter().enumerate() {
        if i < 25 {
            window.push_back(num);
            continue;
        }

        let mut found: bool = false;
        'a: for a in 0..24 {
            for b in a..25 {
                if window[a] + window[b] == num {
                    found = true;
                    break 'a;
                }
            }
        }
        if !found {
            println!("{}", num);
            return;
        }

        window.pop_front();
        window.push_back(num);
    }
}
