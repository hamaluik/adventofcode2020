fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let mut input: Vec<isize> = input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<isize>().unwrap())
        .collect();
    input.sort();
    input.insert(0, 0); // the wall
    input.push(*input.iter().max().unwrap()); // our device

    // had to cheat for this :( but I learned some logic to crack the puzzle
    // for an adapter outputting x jolts, it can have have up to three different
    // joltages plugged into it: x - 1, x - 2, or x - 3 (otherwise, we wouldn't
    // have a valid chain, and since part 1 worked, we have a valid chain).
    // For any given x we just have to work backwards, until we get to the wall;
    // i.e. since x depends on x - 1, x - 2, and x - 3, we know that x - 1 depends
    // on x - 2, x - 3, and x - 4, etc. Where we don't have an adapter (i.e.
    // x - n < 0 or we simply don't have it in the list, x - n should be
    // 0. Use a hashmap to remember it all, going back to 0 / the wall.
    let mut routes_to_n: std::collections::HashMap<isize, isize> =
        std::collections::HashMap::with_capacity(input.len());
    routes_to_n.insert(0, 1);
    for &adapter in input.iter().skip(1) {
        routes_to_n.insert(
            adapter,
            (1..=3)
                .map(|x| {
                    routes_to_n
                        .get(&(adapter - x))
                        .map(|&x| x)
                        .unwrap_or_default()
                })
                .sum(),
        );
    }
    println!("{:?}", routes_to_n.get(routes_to_n.keys().max().unwrap()));
}
