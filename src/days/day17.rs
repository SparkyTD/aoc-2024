use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::utils::solution::{solution, Solution};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum Operand {
    Combo(u8),
    Literal(u8),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum Instruction {
    Adv(Operand), // 0; Divide(combo): reg_a = reg_a / (2 ^ combo)
    Bxl(Operand), // 1; XOR(literal): reg_b = reg_b XOR literal
    Bst(Operand), // 2; Modulo(combo): reg_b = combo % 8
    Jnz(Operand), // 3; Jump(literal): if reg_a != 0 then { ip = literal } // DON'T advance ip after this
    Bxc(Operand), // 4; XOR(ignored): reg_b = reg_b XOR reg_c
    Out(Operand), // 5; OutMod(combo): output <- combo % 8
    Bdv(Operand), // 6; Divide(combo): reg_b = reg_a / (2 ^ combo)
    Cdv(Operand), // 7; Divide(combo): reg_c = reg_a / (2 ^ combo)
}

#[derive(Default, Debug)]
struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    #[allow(dead_code)]
    fn compile(&self) -> Vec<u8> {
        let mut vec = Vec::new();

        for ins in &self.instructions {
            match ins {
                Instruction::Adv(op) => vec.extend(Self::decode_with_operand(0, op)),
                Instruction::Bxl(op) => vec.extend(Self::decode_with_operand(1, op)),
                Instruction::Bst(op) => vec.extend(Self::decode_with_operand(2, op)),
                Instruction::Jnz(op) => vec.extend(Self::decode_with_operand(3, op)),
                Instruction::Bxc(op) => vec.extend(Self::decode_with_operand(4, op)),
                Instruction::Out(op) => vec.extend(Self::decode_with_operand(5, op)),
                Instruction::Bdv(op) => vec.extend(Self::decode_with_operand(6, op)),
                Instruction::Cdv(op) => vec.extend(Self::decode_with_operand(7, op)),
            }
        }

        vec
    }

    #[allow(dead_code)]
    fn decode_with_operand(instruction: u8, operand: &Operand) -> [u8; 2] {
        match operand {
            Operand::Combo(value) => [instruction, *value],
            Operand::Literal(value) => [instruction, *value],
        }
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut jump_targets: HashMap<Instruction, u8> = HashMap::new();
        for ins in self.instructions.iter().cloned() {
            match ins {
                Instruction::Jnz(Operand::Literal(target)) => jump_targets.insert(ins, target),
                _ => continue,
            };
        }

        fn format_operand(operand: &Operand) -> String {
            match operand {
                Operand::Literal(value) => format!("{}", *value),
                Operand::Combo(combo) => {
                    match combo {
                        (0..=3) => format!("{}", *combo),
                        4 => "A".to_string(),
                        5 => "B".to_string(),
                        6 => "C".to_string(),
                        _ => unreachable!(),
                    }
                }
            }
        }

        for ins in self.instructions.iter() {
            let ins_str = match ins {
                Instruction::Adv(op) => format!("A = A / (2 ^ {})", format_operand(op)),
                Instruction::Bxl(op) => format!("B = B xor {}", format_operand(op)),
                Instruction::Bst(op) => format!("B = {} % 8", format_operand(op)),
                Instruction::Jnz(op) => format!("if A != 0: jmp ins_{}", format_operand(op)),
                Instruction::Bxc(_) => "B = B xor C".to_string(),
                Instruction::Out(op) => format!("output.push({} % 8)", format_operand(op)),
                Instruction::Bdv(op) => format!("B = A / (2 ^ {})", format_operand(op)),
                Instruction::Cdv(op) => format!("C = A / (2 ^ {})", format_operand(op)),
            };

            f.write_str(&ins_str)?;
            f.write_str("\n")?;
        }

        Ok(())
    }
}

#[derive(Default, Debug)]
struct Cpu {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,

    ip: usize,
    program: Program,
    output: Vec<u64>,
}

impl Cpu {
    pub fn run_once(&mut self) -> bool {
        let instruction = self.program.instructions.get(self.ip);
        let mut next_ip = self.ip + 1;
        if let Some(instruction) = instruction {
            match instruction {
                Instruction::Adv(op) => {
                    self.reg_a /= 2u64.pow(self.resolve_operand(op) as u32);
                }
                Instruction::Bdv(op) => {
                    self.reg_b = self.reg_a / 2u64.pow(self.resolve_operand(op) as u32);
                }
                Instruction::Cdv(op) => {
                    self.reg_c = self.reg_a / 2u64.pow(self.resolve_operand(op) as u32);
                }
                Instruction::Bxl(op) => {
                    self.reg_b = self.reg_b ^ self.resolve_operand(op);
                }
                Instruction::Bxc(_) => {
                    self.reg_b = self.reg_b ^ self.reg_c;
                }
                Instruction::Bst(op) => {
                    self.reg_b = self.resolve_operand(op) % 8
                }
                Instruction::Jnz(op) => {
                    if self.reg_a != 0 {
                        next_ip = self.resolve_operand(op) as usize;
                    }
                }
                Instruction::Out(op) => {
                    self.output.push(self.resolve_operand(op) % 8);
                }
            }
            self.ip = next_ip;
            true
        } else {
            false
        }
    }

    pub fn run_to_exit(&mut self) {
        while self.run_once() {}
    }

    pub fn reset(&mut self, reg_a: u64, reg_b: u64, reg_c: u64) {
        self.reg_a = reg_a;
        self.reg_b = reg_b;
        self.reg_c = reg_c;
        self.ip = 0;
        self.output.clear();
    }

    pub fn parse_and_load_program(&mut self, input: &Vec<u8>) {
        let mut iter = input.iter();
        while let Some(instr) = iter.next() {
            let instruction = match *instr {
                0 => Instruction::Adv(self.read_operand(&mut iter, true)),
                1 => Instruction::Bxl(self.read_operand(&mut iter, false)),
                2 => Instruction::Bst(self.read_operand(&mut iter, true)),
                3 => Instruction::Jnz(self.read_operand(&mut iter, false)),
                4 => Instruction::Bxc(self.read_operand(&mut iter, false)),
                5 => Instruction::Out(self.read_operand(&mut iter, true)),
                6 => Instruction::Bdv(self.read_operand(&mut iter, true)),
                7 => Instruction::Cdv(self.read_operand(&mut iter, true)),
                _ => panic!("Invalid instruction: {}", instr),
            };
            self.program.instructions.push(instruction);
        }
    }

    fn read_operand(&self, iter: &mut core::slice::Iter<u8>, is_combo: bool) -> Operand {
        match is_combo {
            false => Operand::Literal(*iter.next().unwrap()),
            true => Operand::Combo(*iter.next().unwrap()),
        }
    }

    fn resolve_operand(&self, operand: &Operand) -> u64 {
        match operand {
            Operand::Combo(combo) => {
                match combo {
                    (0..=3) => *combo as u64,
                    4 => self.reg_a,
                    5 => self.reg_b,
                    6 => self.reg_c,
                    _ => panic!("Invalid combo operand: {}", *combo),
                }
            }
            Operand::Literal(value) => *value as u64,
        }
    }
}

fn append(value: &mut u64, digit: u8) {
    *value <<= 3;
    *value |= (digit % 8) as u64;
}

fn find_solution(cpu: &mut Cpu, test_output: &[u8], value: u64) -> Option<u64> {
    if test_output.len() == 0 {
        return Some(value);
    }

    let required_digit = test_output[test_output.len() - 1];

    for i in 0..8 {
        let mut test_value = value;
        append(&mut test_value, i);

        cpu.reset(test_value, 0, 0);
        cpu.run_to_exit();
        let result = &cpu.output;

        let resulting_digit = result.first();
        if let Some(resulting_digit) = resulting_digit {
            if required_digit == *resulting_digit as u8 {
                if let Some(solution) = find_solution(
                    cpu,
                    &test_output[0..test_output.len() - 1],
                    test_value,
                ) {
                    return Some(solution);
                } else {
                    continue;
                }
            }
        }
    }

    None
}

/// <b>Part 1:</b> Simulate a program on a virtual CPU to get the output
/// <br/><br/><b>Part 2:</b> Find the correct value for register A that results in the program outputting itself
#[derive(Default)]
pub struct ChronospatialComputer;

impl Solution for ChronospatialComputer {
    fn solve(&self, input: String) -> (Box<dyn Display>, Box<dyn Display>) {
        let mut cpu = Cpu::default();
        let mut program_original = Vec::new();
        for line in input.lines() {
            if line.starts_with("Register A:") {
                cpu.reg_a = line.split(' ').nth(2).unwrap().parse().unwrap();
            } else if line.starts_with("Register B:") {
                cpu.reg_b = line.split(' ').nth(2).unwrap().parse().unwrap();
            } else if line.starts_with("Register C:") {
                cpu.reg_c = line.split(' ').nth(2).unwrap().parse().unwrap();
            } else if line.starts_with("Program:") {
                let program = line.split(' ').nth(1).unwrap().split(',').map(|x| x.parse().unwrap()).collect();
                cpu.parse_and_load_program(&program);
                program_original.extend(program.iter().map(|b| *b).collect::<Vec<_>>());
                break;
            }
        }

        while cpu.run_once() {}

        let output_joined = cpu.output
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",");

        let value = find_solution(&mut cpu, &program_original, 0).unwrap_or(0);

        solution!(output_joined, value)
    }
}