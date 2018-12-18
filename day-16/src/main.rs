// https://adventofcode.com/2018/day/15

// imports

use std::collections::HashSet;

fn substring(this: &str, start: usize, len: usize) -> String {
    this.chars().skip(start).take(len).collect()
}

#[derive(Debug, Clone)]
enum RegisterID {
    Zero,
    One,
    Two,
    Three,
}

impl RegisterID {
    fn is_valid_register_id(input: i32) -> bool {
        match input {
            0 | 1 | 2 | 3 => true,
            _ => false,
        }
    }

    fn into_register_id(input: i32) -> Option<RegisterID> {
        match input {
            0 => Some(RegisterID::Zero),
            1 => Some(RegisterID::One),
            2 => Some(RegisterID::Two),
            3 => Some(RegisterID::Three),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Registers(i32, i32, i32, i32);

impl Registers {
    fn get(&self, register_id: RegisterID) -> i32 {
        match register_id {
            RegisterID::Zero => self.0,
            RegisterID::One => self.1,
            RegisterID::Two => self.2,
            RegisterID::Three => self.3,
        }
    }

    fn set(&mut self, register_id: RegisterID, value: i32) {
        match register_id {
            RegisterID::Zero => {
                self.0 = value;
            }
            RegisterID::One => {
                self.1 = value;
            }
            RegisterID::Two => {
                self.2 = value;
            }
            RegisterID::Three => {
                self.3 = value;
            }
        }
    }
}

#[derive(Debug, Clone)]
struct OpcodeInstruction(
    i32,        /* input A */
    i32,        /* input B */
    RegisterID, /* output register */
);

impl OpcodeInstruction {
    fn output_register(&self) -> RegisterID {
        return self.2.clone();
    }

    fn input_a(&self) -> i32 {
        return self.0;
    }

    fn input_b(&self) -> i32 {
        return self.1;
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
}

impl Opcode {
    fn all_opcodes() -> HashSet<Opcode> {
        let mut set = HashSet::new();

        set.insert(Opcode::Addr);
        set.insert(Opcode::Addi);

        set.insert(Opcode::Mulr);
        set.insert(Opcode::Muli);

        set.insert(Opcode::Banr);
        set.insert(Opcode::Bani);

        return set;
    }

    fn matches(
        &self,
        mut registers_before: Registers,
        instruction: OpcodeInstruction,
        registers_after: Registers,
    ) -> bool {
        match self {
            Opcode::Addr => {
                // addr (add register) stores into register C the result of adding register A and register B.

                let register_a = RegisterID::into_register_id(instruction.input_a());
                if register_a.is_none() {
                    return false;
                }

                let register_b = RegisterID::into_register_id(instruction.input_b());
                if register_b.is_none() {
                    return false;
                }

                let value_a = registers_before.get(register_a.unwrap());
                let value_b = registers_before.get(register_b.unwrap());

                let register_c = instruction.output_register();

                let result = value_a + value_b;
                registers_before.set(register_c, result);
            }
            Opcode::Addi => {
                // addi (add immediate) stores into register C the result of adding register A and value B.

                let register_a = RegisterID::into_register_id(instruction.input_a());
                if register_a.is_none() {
                    return false;
                }

                let value_a = registers_before.get(register_a.unwrap());
                let value_b = instruction.input_b();

                let result = value_a + value_b;

                let register_c = instruction.output_register();
                registers_before.set(register_c, result);
            }
            Opcode::Mulr => {
                // mulr (multiply register) stores into register C the result of multiplying register A and register B.

                let register_a = RegisterID::into_register_id(instruction.input_a());
                if register_a.is_none() {
                    return false;
                }

                let register_b = RegisterID::into_register_id(instruction.input_b());
                if register_b.is_none() {
                    return false;
                }

                let value_a = registers_before.get(register_a.unwrap());
                let value_b = registers_before.get(register_b.unwrap());

                let register_c = instruction.output_register();

                let result = value_a * value_b;
                registers_before.set(register_c, result);
            }
            Opcode::Muli => {
                // muli (multiply immediate) stores into register C the result of multiplying register A and value B.

                let register_a = RegisterID::into_register_id(instruction.input_a());
                if register_a.is_none() {
                    return false;
                }

                let value_a = registers_before.get(register_a.unwrap());
                let value_b = instruction.input_b();

                let result = value_a * value_b;

                let register_c = instruction.output_register();
                registers_before.set(register_c, result);
            }
            Opcode::Banr => {
                // banr (bitwise AND register) stores into register C the result of the bitwise AND of register A and register B.

                let register_a = RegisterID::into_register_id(instruction.input_a());
                if register_a.is_none() {
                    return false;
                }

                let register_b = RegisterID::into_register_id(instruction.input_b());
                if register_b.is_none() {
                    return false;
                }

                let value_a = registers_before.get(register_a.unwrap());
                let value_b = registers_before.get(register_b.unwrap());

                let register_c = instruction.output_register();

                let result = value_a & value_b;
                registers_before.set(register_c, result);
            }
            Opcode::Bani => {
                // bani (bitwise AND immediate) stores into register C the result of the bitwise AND of register A and value B.

                let register_a = RegisterID::into_register_id(instruction.input_a());
                if register_a.is_none() {
                    return false;
                }

                let value_a = registers_before.get(register_a.unwrap());
                let value_b = instruction.input_b();

                let result = value_a & value_b;

                let register_c = instruction.output_register();
                registers_before.set(register_c, result);
            }
        }

        return registers_before == registers_after;
    }
}

fn part_1(input_string: &str) {
    let mut candidates: Vec<(Registers, OpcodeInstruction, Registers)> = vec![];

    let mut inputs = input_string.trim().lines().peekable();

    while inputs.peek().is_some() {
        let before_register = {
            let before_line = inputs.next().unwrap().trim();

            if !before_line.starts_with("Before: ") {
                continue;
            }

            let mut splitted = before_line.split("Before: ");
            splitted.next();
            let array_like_str: &str = splitted.next().unwrap().trim();

            // remove [ and ] on both ends
            let array_like_str = substring(array_like_str, 1, array_like_str.len() - 2);

            // split and parse i32
            let arr: Vec<i32> = array_like_str
                .split(",")
                .map(|x| x.trim())
                .map(|x| -> i32 { x.parse().unwrap() })
                .collect();

            Registers(arr[0], arr[1], arr[2], arr[3])
        };

        let opcode_instruction = {
            let opcode_instruction_line = inputs.next().unwrap().trim();

            let arr: Vec<i32> = opcode_instruction_line
                .split_whitespace()
                .map(|x| x.trim())
                .map(|x| -> i32 {
                    return x.parse().unwrap();
                })
                .collect();

            OpcodeInstruction(
                arr[1],
                arr[2],
                RegisterID::into_register_id(arr[3]).unwrap(),
            )
        };

        let after_register = {
            let after_line = inputs.next().unwrap().trim();
            assert!(after_line.starts_with("After: "));

            let mut splitted = after_line.split("After: ");
            splitted.next();
            let array_like_str: &str = splitted.next().unwrap().trim();

            // remove [ and ] on both ends
            let array_like_str = substring(array_like_str, 1, array_like_str.len() - 2);

            // split and parse i32
            let arr: Vec<i32> = array_like_str
                .split(",")
                .map(|x| x.trim())
                .map(|x| -> i32 { x.parse().unwrap() })
                .collect();

            Registers(arr[0], arr[1], arr[2], arr[3])
        };

        // println!("{:?}", before_register);
        // println!("{:?}", opcode_instruction);
        // println!("{:?}", after_register);

        candidates.push((before_register, opcode_instruction, after_register));
    }

    let candidates = candidates;

    let remaining: Vec<(Registers, OpcodeInstruction, Registers)> = candidates
        .into_iter()
        .filter(|(before_register, opcode_instruction, after_register)| {
            let opcodes = Opcode::all_opcodes();

            let matched_opcodes: Vec<Opcode> = opcodes
                .into_iter()
                .filter(|opcode| {
                    return opcode.matches(
                        before_register.clone(),
                        opcode_instruction.clone(),
                        after_register.clone(),
                    );
                })
                .collect();

            return matched_opcodes.len() >= 3;
        })
        .collect();

    println!("Part 1: {}", remaining.len());
}

fn main() {
    let input_string = include_str!("input.txt");

    part_1(input_string);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_matches() {
        {
            let opcode = Opcode::Addr;

            // valid

            let registers_before = Registers(1, 2, 3, 4);
            let instruction = OpcodeInstruction(0, 1, RegisterID::Zero);
            let registers_after = Registers(3, 2, 3, 4);
            let result = opcode.matches(
                registers_before.clone(),
                instruction.clone(),
                registers_after,
            );

            assert_eq!(result, true);

            // invalid

            let registers_after = Registers(0, 2, 3, 4);
            let result = opcode.matches(
                registers_before.clone(),
                instruction.clone(),
                registers_after,
            );

            assert_eq!(result, false);
        }
    }

}
