struct State {
    x: isize,
    y: isize,
    angle: isize,
}

fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();

    let state = State {
        x: 0,
        y: 0,
        angle: 0,
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
                'N' => state.y += arg,
                'E' => state.x += arg,
                'S' => state.y -= arg,
                'W' => state.x -= arg,
                'L' => state.angle += arg,
                'R' => state.angle -= arg,
                'F' => {
                    // normalize the angle
                    while state.angle < 0 {
                        state.angle += 360;
                    }
                    while state.angle >= 360 {
                        state.angle -= 360;
                    }
                    match state.angle {
                        0 => state.x += arg,
                        90 => state.y += arg,
                        180 => state.x -= arg,
                        270 => state.y -= arg,
                        _ => panic!("bad direction"),
                    }
                }
                _ => panic!("bad instruction"),
            }
            state
        });

    println!("{}", state.x.abs() + state.y.abs());
}
