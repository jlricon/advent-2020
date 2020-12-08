use im_rc::Vector;
use std::collections::HashSet;
#[derive(Clone, Copy, Debug)]
enum Instruction {
    Nop,
    Acc(i32),
    Jmp(i32),
}

#[derive(Debug)]
enum Errors {
    EndOfFile,
    UnsafeInstCounterSet,
}

struct VM {
    inst_counter: usize,
    program: Vector<Instruction>,
    prog_len: usize,
    acc: i32,
}
impl VM {
    fn safe_next_inst_counter(&self, n: i32) -> Result<usize, Errors> {
        // inst_counter cannot be <0 or > prog_len
        let proposed_value = self.inst_counter as i32 + n;
        if proposed_value > self.prog_len as i32 {
            Err(Errors::EndOfFile)
        } else if proposed_value < 0 {
            Err(Errors::UnsafeInstCounterSet)
        } else {
            Ok(proposed_value as usize)
        }
    }
    fn new(program: Vector<Instruction>) -> VM {
        let prog_len = program.len();
        VM {
            inst_counter: 0,
            program,
            prog_len,
            acc: 0,
        }
    }
    fn print_state(&self) {
        println!("Counter at {}, acc={}", self.inst_counter, self.acc);
    }
    fn step(&self) -> Result<VM, Errors> {
        let inst = self.program.get(self.inst_counter as usize);
        match inst {
            Some(Instruction::Nop) => Ok(VM {
                inst_counter: self.inst_counter + 1,
                program: self.program.clone(),
                ..*self
            }),
            Some(Instruction::Acc(val)) => Ok(VM {
                inst_counter: self.inst_counter + 1,
                program: self.program.clone(),
                acc: self.acc + val,
                ..*self
            }),
            Some(Instruction::Jmp(val)) => match self.safe_next_inst_counter(*val) {
                Ok(maybe_next) => Ok(VM {
                    inst_counter: maybe_next,
                    program: self.program.clone(),
                    ..*self
                }),
                Err(e) => Err(e),
            },
            None => Err(Errors::EndOfFile),
        }
    }
}
fn main() {
    // Part 1 and 2 together. Being sneaky and manually changing the relevant input value :P
    let input: Vector<Instruction> = include_str!("../../input/day8.txt")
        .lines()
        .map(|l| {
            let mut splitted = l.split_whitespace();
            let inst = splitted.nth(0).unwrap();
            let val: i32 = splitted.nth(0).unwrap().parse().unwrap();
            match inst {
                "nop" => Instruction::Nop,
                "acc" => Instruction::Acc(val),
                "jmp" => Instruction::Jmp(val),
                _ => panic!(),
            }
        })
        .collect();
    let mut machine = VM::new(input);
    let mut prev_counters: Vec<usize> = vec![0];
    let mut executed_so_far: HashSet<usize> = HashSet::new();

    // Part 1
    loop {
        if executed_so_far.contains(&machine.inst_counter) {
            println!(
                "Early termination, ended up at {} coming from {} after running {:?}",
                &machine.inst_counter,
                prev_counters[prev_counters.len() - 2],
                &machine.program[*prev_counters.last().unwrap()]
            );
            machine.print_state();
            break;
        }
        let step_result = machine.step();

        match step_result {
            Ok(new_machine) => {
                executed_so_far.insert(machine.inst_counter);
                prev_counters.push(new_machine.inst_counter);
                machine = new_machine;
            }
            Err(Errors::UnsafeInstCounterSet) => {
                machine.print_state();
                panic!("Attempted unsafe instruction counter")
            }
            Err(Errors::EndOfFile) => {
                machine.print_state();
                panic!("Reached end of file")
            }
        }
    }
    let instruction_causing_loop = machine.program[prev_counters.len() - 2];
}
