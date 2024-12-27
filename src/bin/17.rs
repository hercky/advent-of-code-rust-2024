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
struct Instruction {
    code: Code,
    operand: u8,
    operand_type: OperandType,
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

                // Process pairs of numbers as (opcode, operand)
                program = numbers
                    .chunks(2)
                    .map(|chunk| Instruction {
                        code: Code::from(chunk[0]),
                        operand: chunk[1],
                        operand_type: OperandType::from(Code::from(chunk[0])),
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

    fn get_register(&self, reg: char) -> Option<usize> {
        match reg {
            'A' => Some(self.a.val),
            'B' => Some(self.b.val),
            'C' => Some(self.c.val),
            _ => None,
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

    fn one_step(&mut self) -> Option<u8> {
        if self.halt {
            return None;
        }

        let instruction = self.program[self.counter];

        let op_val = match instruction.operand_type {
            OperandType::Combo => self.get_combo_value(instruction.operand),
            OperandType::Literal => instruction.operand as usize,
            _ => 0,
        };

        println!("Instruction: {:?}", instruction);
        println!("Op Val: {}", op_val);

        match instruction.code {
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
                    self.counter = op_val as usize - 1;
                }
            }
            Code::bxc => {
                self.b.val = self.b.val ^ self.c.val;
            }
            Code::out => {
                print!("{},", op_val % 8);
            }
            Code::bdv => {
                self.b.val = self.a.val as usize / 2usize.pow(op_val as u32);
            }
            Code::cdv => {
                self.c.val = self.b.val as usize / 2usize.pow(op_val as u32);
            }
        }

        self.counter += 1;
        if self.counter >= self.program.len() {
            self.halt = true;
        }

        None
    }

    fn print_state(&self) {
        println!("Register A: {}", self.a.val);
        println!("Register B: {}", self.b.val);
        println!("Register C: {}", self.c.val);
        println!("Program Counter: {}", self.counter);
        println!("Halt: {}", self.halt);
    }

    fn run(&mut self) {
        while !self.halt {
            println!("--------------------------------");
            self.print_state();
            self.one_step();
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut machine = Machine::parse_and_init(input);
    machine.run();
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
