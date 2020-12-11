#[derive(Clone)]
enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

fn program_halts(instructions: Vec<Instruction>) -> (bool, isize) {
    let mut accumulator: isize = 0;
    let mut ip: isize = 0;

    use std::collections::HashSet;
    let mut instructions_that_ran: HashSet<isize> = HashSet::with_capacity(instructions.len());

    loop {
        let new_instruction = instructions_that_ran.insert(ip);
        if !new_instruction {
            return (false, accumulator);
        }
        if (ip as usize) >= instructions.len() {
            return (true, accumulator);
        }

        match instructions[ip as usize] {
            Instruction::Acc(d) => {
                accumulator += d;
                ip += 1;
            }
            Instruction::Jmp(d) => {
                ip += d;
            }
            Instruction::Nop(_) => {
                ip += 1;
            }
        }
    }
}

fn main() {
    let instructions = std::fs::read_to_string("../input.txt").unwrap();
    let instructions: Vec<Instruction> = instructions
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|line| {
            let mut parts = line.split(" ");
            let inst = parts.next().unwrap();
            let arg: isize = parts.next().unwrap().parse().unwrap();
            match inst {
                "acc" => Instruction::Acc(arg),
                "jmp" => Instruction::Jmp(arg),
                "nop" => Instruction::Nop(arg),
                _ => panic!("unhandled instruction"),
            }
        })
        .collect();

    for (i, instruction) in instructions.iter().enumerate() {
        match instruction {
            Instruction::Acc(_) => continue, // acc instructions not modified,
            Instruction::Jmp(d) => {
                let mut instructions = instructions.clone();
                instructions[i] = Instruction::Nop(*d);
                let (halts, acc) = program_halts(instructions);
                if halts {
                    println!("{}", acc);
                    return;
                }
            }
            Instruction::Nop(d) => {
                let mut instructions = instructions.clone();
                instructions[i] = Instruction::Jmp(*d);
                let (halts, acc) = program_halts(instructions);
                if halts {
                    println!("{}", acc);
                    return;
                }
            }
        }
    }
    println!("no working modifications found?");
}
