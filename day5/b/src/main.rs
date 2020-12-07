fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let mut tickets: Vec<i32> = input
        .split("\n")
        .filter(|f| !f.is_empty())
        .map(|seat| {
            let mut rows = 0..128;
            for c in seat.chars() {
                let len = (rows.end - rows.start) / 2;
                let start = match c {
                    'F' => rows.start,
                    'B' => rows.end - len,
                    _ => break,
                };
                rows = start..(start + len);
            }
            let row = rows.start;

            let mut cols = 0..8;
            for c in seat.chars().skip(7) {
                let len = (cols.end - cols.start) / 2;
                let start = match c {
                    'L' => cols.start,
                    'R' => cols.end - len,
                    _ => break,
                };
                cols = start..(start + len);
            }
            let col = cols.start;

            (row * 8) + col
        })
        .collect();
    tickets.sort();
    //for i in 1..(tickets.len() - 1) {
    //if tickets[i - 1]
    //}
    println!(
        "{:?}",
        tickets
            .iter()
            .zip(tickets.iter().skip(1))
            .find(|(&a, &b)| { b != (a + 1) })
    );
}
