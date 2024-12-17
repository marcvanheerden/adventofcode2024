const REGISTERS: usize = 3;

#[derive(Debug)]
struct Computer {
    registers: [u64; REGISTERS],
    program: Vec<u64>,
    output: Vec<u64>,
    instruction_pointer: usize,
}

impl Computer {
    fn new(input: &str) -> Option<Self> {
        let mut lines = input.lines();
        let mut registers = [0; REGISTERS];
        for reg in registers.iter_mut() {
            *reg = lines.next()?.split(": ").nth(1)?.parse().unwrap();
        }

        let program = lines
            .nth(1)?
            .split(": ")
            .nth(1)?
            .split(',')
            .filter_map(|s| s.parse::<u64>().ok())
            .collect();

        let instruction_pointer = 0;

        let output = Vec::new();
        Some(Computer {
            registers,
            program,
            output,
            instruction_pointer,
        })
    }

    fn step_over(&mut self) -> bool {
        if self.instruction_pointer >= self.program.len() - 1 {
            return false;
        }

        let opcode = self.program[self.instruction_pointer];
        let raw_operand = self.program[self.instruction_pointer + 1];

        let operand = match (opcode, raw_operand) {
            (1 | 3, _) => raw_operand,
            (_, 0..=3) => raw_operand,
            (_, 4) => self.registers[0],
            (_, 5) => self.registers[1],
            (_, 6) => self.registers[2],
            (_, _) => unreachable!(),
        };

        match opcode {
            0 => {
                self.registers[0] /= 2u64.pow(operand as u32);
                self.instruction_pointer += 2;
            }
            1 => {
                self.registers[1] ^= operand;
                self.instruction_pointer += 2;
            }
            2 => {
                self.registers[1] = operand % 8;
                self.instruction_pointer += 2;
            }
            3 => {
                if self.registers[0] == 0 {
                    self.instruction_pointer += 2;
                } else {
                    self.instruction_pointer = operand as usize;
                }
            }
            4 => {
                self.registers[1] ^= self.registers[2];
                self.instruction_pointer += 2;
            }
            5 => {
                self.output.push(operand % 8);
                self.instruction_pointer += 2;
            }
            6 => {
                self.registers[1] = self.registers[0] / 2u64.pow(operand as u32);
                self.instruction_pointer += 2;
            }
            _ => {
                self.registers[2] = self.registers[0] / 2u64.pow(operand as u32);
                self.instruction_pointer += 2;
            }
        }

        true
    }

    fn run(&mut self) {
        let mut halted = false;
        while !halted {
            halted = !self.step_over();
        }
    }

    fn quiness(&mut self) -> usize {
        let mut halted = false;
        while !halted {
            halted = !self.step_over();
            if self.output != self.program[0..std::cmp::min(self.output.len(), self.program.len())]
            {
                return self.output.len().saturating_sub(1);
            }
        }

        for idx in 0..std::cmp::min(self.output.len(), self.program.len()) {
            if self.output[idx] != self.program[idx] {
                return idx;
            }
        }
        std::cmp::min(self.output.len(), self.program.len())
    }

    fn print_output(&self) -> String {
        self.output
            .iter() // Iterate over the vector
            .map(|n| n.to_string()) // Convert each integer to a string
            .collect::<Vec<_>>() // Collect into a temporary Vec<String>
            .join(",")
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let mut cmp = Computer::new(input).unwrap();
    cmp.run();
    dbg!(cmp.print_output());

    let mut candidates = Vec::new();
    for val in 0..=1023 {
        let mut cmp = Computer::new(input).unwrap();
        cmp.registers[0] = val;
        if cmp.quiness() > 0 {
            candidates.push(val);
        }
    }

    for completion in 1..=15 {
        candidates = candidates
            .into_iter()
            .flat_map(|c| {
                (0..8)
                    .filter_map(|nc| {
                        let mut cmp = Computer::new(input).unwrap();
                        let new_candidate = (nc << (7 + 3 * completion)) + c;
                        cmp.registers[0] = new_candidate;
                        if cmp.quiness() > completion {
                            Some(new_candidate)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<u64>>()
            })
            .collect::<Vec<u64>>();
    }

    dbg!(candidates.into_iter().min().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whole_program() {
        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

        let mut cmp = Computer::new(input).unwrap();
        cmp.run();
        assert_eq!(cmp.print_output(), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_opcode2() {
        let mut cmp = Computer {
            registers: [0, 0, 9],
            program: vec![2, 6],
            output: Vec::new(),
            instruction_pointer: 0,
        };
        cmp.step_over();

        assert_eq!(cmp.registers[1], 1);
    }

    #[test]
    fn test_small_program() {
        let mut cmp = Computer {
            registers: [10, 0, 0],
            program: vec![5, 0, 5, 1, 5, 4],
            output: Vec::new(),
            instruction_pointer: 0,
        };
        cmp.run();

        assert_eq!(cmp.output, vec![0, 1, 2]);
    }

    #[test]
    fn test_small_program2() {
        let mut cmp = Computer {
            registers: [2024, 0, 0],
            program: vec![0, 1, 5, 4, 3, 0],
            output: Vec::new(),
            instruction_pointer: 0,
        };
        cmp.run();

        assert_eq!(cmp.output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(cmp.registers[0], 0);
    }

    #[test]
    fn test_small_program3() {
        let mut cmp = Computer {
            registers: [0, 29, 0],
            program: vec![1, 7],
            output: Vec::new(),
            instruction_pointer: 0,
        };
        cmp.step_over();

        assert_eq!(cmp.registers[1], 26);
    }
}

//    If register B contains 29, the program 1,7 would set register B to 26.
//    If register B contains 2024 and register C contains 43690, the program 4,0 would set register B to 44354.
//
//
//  Program: 2,4,1,1,7,5,0,3,1,4,4,5,5,5,3,0
//  2, 4   B = A % 8
//  1, 1   B = B ^ 1
//  7, 5   C = A / 2.pow(B)
//  0, 3   A = A / 2.pow(3)
//  1, 4   B = B ^ 4
//  4, 5   B = B ^ C
//  5, 5   output B % 8
//  3, 0   if A == 0 then end else jump to start
//
//
//  B = (A % 8) ^ 1
//  C = A >> B
//  A = A >> 3
//  (B XOR 4) XOR C % 8
//
//    CCCAAABBB
//    000101111
//
//    we want c = a / 2.pow(6)
//    6 = 110
//    so A = ...111
//
//    B = 111
//    B = 110
//    C = whatever we want
//    B = 010
//    B = 010 = 2
//
//    A = 101
//    B = 100
//    C =
//
