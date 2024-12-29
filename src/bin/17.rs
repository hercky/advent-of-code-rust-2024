advent_of_code::solution!(17);
use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Register {
    val: usize,
    name: char,
}

impl Register {
    fn new(name: char, val: usize) -> Self {
        Self { val, name }
    }
}

#[derive(Debug, Clone, Copy)]
enum Code {
    adv,
    bxl,
    bst,
    jnz,
    bxc,
    out,
    bdv,
    cdv,
}

impl From<u8> for Code {
    fn from(code: u8) -> Self {
        match code {
            0 => Code::adv,
            1 => Code::bxl,
            2 => Code::bst,
            3 => Code::jnz,
            4 => Code::bxc,
            5 => Code::out,
            6 => Code::bdv,
            7 => Code::cdv,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum OperandType {
    Combo,
    Literal,
    Nil,
}

impl From<Code> for OperandType {
    fn from(code: Code) -> Self {
        match code {
            Code::adv => OperandType::Combo,
            Code::bxl => OperandType::Literal,
            Code::bst => OperandType::Combo,
            Code::jnz => OperandType::Literal,
            Code::bxc => OperandType::Nil,
            Code::out => OperandType::Combo,
            Code::bdv => OperandType::Combo,
            Code::cdv => OperandType::Combo,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct CodeInstruction {
    code: Code,
}

#[derive(Debug, Clone, Copy)]
struct OperandInstruction {
    operand: u8,
    operand_type: OperandType,
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Code(CodeInstruction),
    Operand(OperandInstruction),
}

struct Machine {
    a: Register,
    b: Register,
    c: Register,
    program: Vec<Instruction>,
    halt: bool,
    counter: usize,
}

impl Machine {
    fn parse_and_init(input: &str) -> Self {
        // Parse register initializations
        let reg_pattern = Regex::new(r"Register ([A-C]): (\d+)").unwrap();
        let mut a = Register::new('A', 0);
        let mut b = Register::new('B', 0);
        let mut c = Register::new('C', 0);

        // Parse program instructions
        let prog_pattern = Regex::new(r"Program: ([\d,]+)").unwrap();
        let mut program = Vec::new();

        for line in input.lines() {
            if let Some(caps) = reg_pattern.captures(line) {
                let reg_name = caps[1].chars().next().unwrap();
                let value = caps[2].parse::<i32>().unwrap();
                match reg_name {
                    'A' => a = Register::new('A', value as usize),
                    'B' => b = Register::new('B', value as usize),
                    'C' => c = Register::new('C', value as usize),
                    _ => unreachable!(),
                }
            } else if let Some(caps) = prog_pattern.captures(line) {
                let numbers: Vec<u8> = caps[1]
                    .split(',')
                    .map(|n| n.parse::<u8>().unwrap())
                    .collect();

                // Process numbers alternating between code and operand
                program = numbers
                    .chunks(2)
                    .flat_map(|chunk| {
                        let code = Code::from(chunk[0]);
                        [
                            Instruction::Code(CodeInstruction { code }),
                            Instruction::Operand(OperandInstruction {
                                operand: chunk[1],
                                operand_type: OperandType::from(code),
                            }),
                        ]
                    })
                    .collect();
            }
        }

        Self {
            a,
            b,
            c,
            program,
            halt: false,
            counter: 0,
        }
    }

    fn get_combo_value(&self, operand: u8) -> usize {
        match operand {
            0..4 => operand as usize,
            4 => self.a.val,
            5 => self.b.val,
            6 => self.c.val,
            _ => unreachable!(),
        }
    }

    fn one_step(&mut self, opcode: CodeInstruction, operand: OperandInstruction) -> Option<u8> {
        if self.halt {
            return None;
        }

        let op_val = match operand.operand_type {
            OperandType::Combo => self.get_combo_value(operand.operand),
            OperandType::Literal => operand.operand as usize,
            OperandType::Nil => 0,
        };

        // println!("Instruction\t: Opcode {:?}\tOperand {:?}", opcode, operand);
        // println!("Op Val: {}", op_val);

        let mut jump = false;
        let mut print_output = None;

        match opcode.code {
            Code::adv => {
                self.a.val = self.a.val as usize / 2usize.pow(op_val as u32);
            }
            Code::bxl => {
                self.b.val = self.b.val ^ op_val;
            }
            Code::bst => {
                self.b.val = op_val % 8;
            }
            Code::jnz => {
                if self.a.val != 0 {
                    self.counter = op_val as usize;
                    jump = true;
                }
            }
            Code::bxc => {
                self.b.val = self.b.val ^ self.c.val;
            }
            Code::out => {
                // print!("{},", op_val % 8);
                print_output = Some((op_val % 8) as u8);
            }
            Code::bdv => {
                self.b.val = self.a.val as usize / 2usize.pow(op_val as u32);
            }
            Code::cdv => {
                self.c.val = self.a.val as usize / 2usize.pow(op_val as u32);
            }
        }

        if !jump {
            self.counter += 2;
        }

        if self.counter >= self.program.len() {
            self.halt = true;
        }

        print_output
    }

    fn print_state(&self) {
        println!("Register A: {}", self.a.val);
        println!("Register B: {}", self.b.val);
        println!("Register C: {}", self.c.val);
        println!("Program Counter: {}", self.counter);
        println!("Halt: {}", self.halt);
    }

    fn run(&mut self) {
        // self.print_state();
        // println!("{:?}", self.program);
        let mut result: Vec<u8> = Vec::new();

        while !self.halt {
            // println!("--------------------------------");

            let Instruction::Code(code) = self.program[self.counter] else {
                unreachable!()
            };
            let Instruction::Operand(operand) = self.program[self.counter + 1] else {
                unreachable!()
            };

            if let Some(print_output) = self.one_step(code, operand) {
                result.push(print_output);
            }
        }

        println!("--------------------------------");
        println!("{:?}", result);
    }

    fn run_and_get_output(&mut self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        while !self.halt {
            let Instruction::Code(code) = self.program[self.counter] else {
                unreachable!()
            };
            let Instruction::Operand(operand) = self.program[self.counter + 1] else {
                unreachable!()
            };

            if let Some(print_output) = self.one_step(code, operand) {
                result.push(print_output);
            }
        }

        result
    }

    fn reset(&mut self, a: usize, b: usize, c: usize) {
        self.a = Register::new('A', a);
        self.b = Register::new('B', b);
        self.c = Register::new('C', c);
        self.counter = 0;
        self.halt = false;
    }

    fn program_to_vec(&self) -> Vec<u8> {
        self.program
            .iter()
            .map(|i| match i {
                Instruction::Code(code) => code.code as u8,
                Instruction::Operand(operand) => operand.operand,
            })
            .collect()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut machine = Machine::parse_and_init(input);

    machine.run();

    None
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut machine = Machine::parse_and_init(input);

    let target_sequence = machine.program_to_vec();
    let n = target_sequence.len();

    let mut a: usize = 0;
    for i in (0..n).rev() {
        a <<= 3;

        loop {
            machine.reset(a, 0, 0);
            let output = machine.run_and_get_output();

            if output == target_sequence[i..] {
                break;
            }
            a += 1;
        }
    }

    println!("--------------part two---------------");
    println!("{:?}", target_sequence);
    println!("--------------------------------");
    println!("{}", a);
    println!("--------------------------------");

    Some(a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(117440));
    }
}
