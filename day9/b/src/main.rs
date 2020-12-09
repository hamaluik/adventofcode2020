const PREAMBLE: usize = 25;

fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    //let input = "35
    //20
    //15
    //25
    //47
    //40
    //62
    //55
    //65
    //95
    //102
    //117
    //150
    //182
    //127
    //219
    //299
    //277
    //309
    //576";
    let lines: Vec<usize> = input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect();

    let mut window: std::collections::VecDeque<usize> =
        std::collections::VecDeque::with_capacity(25);
    let mut target: Option<usize> = None;
    'lines: for (i, &num) in lines.iter().enumerate() {
        if i < PREAMBLE {
            window.push_back(num);
            continue;
        }

        let mut found: bool = false;
        'a: for a in 0..(PREAMBLE - 1) {
            for b in (a + 1)..PREAMBLE {
                if window[a] + window[b] == num {
                    found = true;
                    break 'a;
                }
            }
        }
        if !found {
            target = Some(num);
            break 'lines;
        }

        window.pop_front();
        window.push_back(num);
    }

    let target = target.unwrap();
    for start_index in 0..(lines.len() - 1) {
        for len in 2..(lines.len() - start_index) {
            //for n in lines.iter().skip(start_index).take(len) {
            //print!("{} + ", n);
            //}
            let sum: usize = lines.iter().skip(start_index).take(len).sum();
            //println!(" = {}", sum);
            if sum == target {
                let smallest: usize = *lines.iter().skip(start_index).take(len).min().unwrap();
                let largest: usize = *lines.iter().skip(start_index).take(len).max().unwrap();
                println!("{}", smallest + largest);

                return;
            }
        }
        //println!();
    }
}
