enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop,
}

fn main() {
    let instructions = std::fs::read_to_string("../input.txt").unwrap();
    let instructions: Vec<Instruction> = instructions
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|line| {
            let mut parts = line.split(" ");
            let inst = parts.next().unwrap();
            //let arg = parts.next().unwrap();
            match inst {
                "acc" => Instruction::Acc(parts.next().unwrap().parse().unwrap()),
                "jmp" => Instruction::Jmp(parts.next().unwrap().parse().unwrap()),
                "nop" => Instruction::Nop,
                _ => panic!("unhandled instruction"),
            }
        })
        .collect();

    let mut accumulator: isize = 0;
    let mut ip: isize = 0;

    use std::collections::HashSet;
    let mut instructions_that_ran: HashSet<isize> = HashSet::with_capacity(instructions.len());

    loop {
        let new_instruction = instructions_that_ran.insert(ip);
        if !new_instruction {
            println!("{}", accumulator);
            break;
        }
        match instructions[ip as usize] {
            Instruction::Acc(d) => {
                accumulator += d;
                ip += 1;
            }
            Instruction::Jmp(d) => {
                ip += d;
            }
            Instruction::Nop => {
                ip += 1;
            }
        }
    }
}
