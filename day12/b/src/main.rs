struct State {
    x: isize,
    y: isize,
    wx: isize,
    wy: isize,
}

fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();

    let state = State {
        x: 0,
        y: 0,
        wx: 10,
        wy: 1,
    };

    let state = input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| {
            let instruction = l.chars().nth(0).unwrap();
            let arg: isize = l[1..].parse().unwrap();
            (instruction, arg)
        })
        .fold(state, |mut state, (instruction, arg)| {
            match instruction {
                'N' => state.wy += arg,
                'E' => state.wx += arg,
                'S' => state.wy -= arg,
                'W' => state.wx -= arg,
                'L' => {
                    for _ in 0..(arg / 90) {
                        let a = state.wy;
                        state.wy = state.wx;
                        state.wx = -a;
                    }
                }
                'R' => {
                    for _ in 0..(arg / 90) {
                        let a = state.wy;
                        state.wy = -state.wx;
                        state.wx = a;
                    }
                }
                'F' => {
                    state.x += state.wx * arg;
                    state.y += state.wy * arg;
                }
                _ => panic!("bad instruction"),
            }
            state
        });

    println!("{}", state.x.abs() + state.y.abs());
}
