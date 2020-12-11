fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let mut input: Vec<usize> = input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect();
    input.sort();
    input.push(*input.iter().max().unwrap() + 3); // our device

    #[derive(Default, Debug)]
    struct Accum {
        joltage: usize,
        counts: [usize; 3],
    };

    let accum = input.iter().fold(Accum::default(), |mut accum, adapter| {
        let delta = adapter - accum.joltage;
        assert!(delta >= 1);
        assert!(delta <= 3);
        //println!("{:?}: {} | {}", accum, delta, adapter);
        accum.counts[delta - 1] += 1;
        accum.joltage = *adapter;
        accum
    });

    println!("{:?}", accum);
    let answer = accum.counts[0] * accum.counts[2];
    println!("{}", answer);
}
