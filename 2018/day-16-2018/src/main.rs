// https://adventofcode.com/2018/day/16

// imports

use std::collections::HashMap;
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
    i32,        /* opcode number */
    i32,        /* input A */
    i32,        /* input B */
    RegisterID, /* output register */
);

impl OpcodeInstruction {
    fn output_register(&self) -> RegisterID {
        self.3.clone()
    }

    fn input_a(&self) -> i32 {
        self.1
    }

    fn input_b(&self) -> i32 {
        self.2
    }

    fn input_opcode_number(&self) -> i32 {
        self.0
    }
}

type OpCodeMap = HashMap<i32, Opcode>;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
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

        set.insert(Opcode::Borr);
        set.insert(Opcode::Bori);

        set.insert(Opcode::Setr);
        set.insert(Opcode::Seti);

        set.insert(Opcode::Gtir);
        set.insert(Opcode::Gtri);
        set.insert(Opcode::Gtrr);

        set.insert(Opcode::Eqir);
        set.insert(Opcode::Eqri);
        set.insert(Opcode::Eqrr);

        set
    }

    fn matches(
        &self,
        registers_before: Registers,
        instruction: OpcodeInstruction,
        registers_after_expected: Registers,
    ) -> bool {
        match self.execute(registers_before, instruction) {
            None => false,
            Some(registers_after_actual) => registers_after_expected == registers_after_actual,
        }
    }

    fn execute(
        &self,
        mut registers_before: Registers,
        instruction: OpcodeInstruction,
    ) -> Option<Registers> {
        match self {
            Opcode::Addr => {
                // addr (add register) stores into register C the result of adding register A and register B.

                let register_a = RegisterID::into_register_id(instruction.input_a());
                register_a.as_ref()?;

                let register_b = RegisterID::into_register_id(instruction.input_b());
                register_b.as_ref()?;

                let value_a = registers_before.get(register_a.unwrap());
                let value_b = registers_before.get(register_b.unwrap());

                let register_c = instruction.output_register();

                let result = value_a + value_b;
                registers_before.set(register_c, result);
            }
            Opcode::Addi => {
                // addi (add immediate) stores into register C the result of adding register A and value B.

                let register_a = RegisterID::into_register_id(instruction.input_a());
                register_a.as_ref()?;

                let value_a = registers_before.get(register_a.unwrap());
                let value_b = instruction.input_b();

                let result = value_a + value_b;

                let register_c = instruction.output_register();
                registers_before.set(register_c, result);
            }
            Opcode::Mulr => {
                // mulr (multiply register) stores into register C the result of multiplying register A and register B.

                let register_a = RegisterID::into_register_id(instruction.input_a());
                register_a.as_ref()?;

                let register_b = RegisterID::into_register_id(instruction.input_b());
                register_b.as_ref()?;

                let value_a = registers_before.get(register_a.unwrap());
                let value_b = registers_before.get(register_b.unwrap());

                let register_c = instruction.output_register();

                let result = value_a * value_b;
                registers_before.set(register_c, result);
            }
            Opcode::Muli => {
                // muli (multiply immediate) stores into register C the result of multiplying register A and value B.

                let register_a = RegisterID::into_register_id(instruction.input_a());
                register_a.as_ref()?;

                let value_a = registers_before.get(register_a.unwrap());
                let value_b = instruction.input_b();

                let result = value_a * value_b;

                let register_c = instruction.output_register();
                registers_before.set(register_c, result);
            }
            Opcode::Banr => {
                // banr (bitwise AND register) stores into register C the result of the bitwise AND of register A and register B.

                let register_a = RegisterID::into_register_id(instruction.input_a());
                register_a.as_ref()?;

                let register_b = RegisterID::into_register_id(instruction.input_b());
                register_b.as_ref()?;

                let value_a = registers_before.get(register_a.unwrap());
                let value_b = registers_before.get(register_b.unwrap());

                let register_c = instruction.output_register();

                let result = value_a & value_b;
                registers_before.set(register_c, result);
            }
            Opcode::Bani => {
                // bani (bitwise AND immediate) stores into register C the result of the bitwise AND of register A and value B.

                let register_a = RegisterID::into_register_id(instruction.input_a());
                register_a.as_ref()?;

                let value_a = registers_before.get(register_a.unwrap());
                let value_b = instruction.input_b();

                let result = value_a & value_b;

                let register_c = instruction.output_register();
                registers_before.set(register_c, result);
            }
            Opcode::Borr => {
                // borr (bitwise OR register) stores into register C the result of the bitwise OR of register A and register B.

                let register_a = RegisterID::into_register_id(instruction.input_a());
                register_a.as_ref()?;

                let register_b = RegisterID::into_register_id(instruction.input_b());
                register_b.as_ref()?;

                let value_a = registers_before.get(register_a.unwrap());
                let value_b = registers_before.get(register_b.unwrap());

                let register_c = instruction.output_register();

                let result = value_a | value_b;
                registers_before.set(register_c, result);
            }
            Opcode::Bori => {
                // bori (bitwise OR immediate) stores into register C the result of the bitwise OR of register A and value B.

                let register_a = RegisterID::into_register_id(instruction.input_a());
                register_a.as_ref()?;

                let value_a = registers_before.get(register_a.unwrap());
                let value_b = instruction.input_b();

                let result = value_a | value_b;

                let register_c = instruction.output_register();
                registers_before.set(register_c, result);
            }
            Opcode::Setr => {
                // setr (set register) copies the contents of register A into register C. (Input B is ignored.)

                let register_a = RegisterID::into_register_id(instruction.input_a());
                register_a.as_ref()?;

                let value_a = registers_before.get(register_a.unwrap());

                let register_c = instruction.output_register();
                registers_before.set(register_c, value_a);
            }
            Opcode::Seti => {
                // seti (set immediate) stores value A into register C. (Input B is ignored.)

                let value_a = instruction.input_a();

                let register_c = instruction.output_register();
                registers_before.set(register_c, value_a);
            }
            Opcode::Gtir => {
                // gtir (greater-than immediate/register) sets register C to 1 if value A is greater than register B.
                // Otherwise, register C is set to 0.

                let value_a = instruction.input_a();

                let register_b = RegisterID::into_register_id(instruction.input_b());
                register_b.as_ref()?;
                let value_b = registers_before.get(register_b.unwrap());

                let result = if value_a > value_b { 1 } else { 0 };

                let register_c = instruction.output_register();
                registers_before.set(register_c, result);
            }
            Opcode::Gtri => {
                // gtri (greater-than register/immediate) sets register C to 1 if register A is greater than value B.
                // Otherwise, register C is set to 0.

                let register_a = RegisterID::into_register_id(instruction.input_a());
                register_a.as_ref()?;
                let value_a = registers_before.get(register_a.unwrap());

                let value_b = instruction.input_b();

                let result = if value_a > value_b { 1 } else { 0 };

                let register_c = instruction.output_register();
                registers_before.set(register_c, result);
            }
            Opcode::Gtrr => {
                // gtrr (greater-than register/register) sets register C to 1 if register A is greater than register B.
                // Otherwise, register C is set to 0.

                let register_a = RegisterID::into_register_id(instruction.input_a());
                register_a.as_ref()?;
                let value_a = registers_before.get(register_a.unwrap());

                let register_b = RegisterID::into_register_id(instruction.input_b());
                register_b.as_ref()?;
                let value_b = registers_before.get(register_b.unwrap());

                let result = if value_a > value_b { 1 } else { 0 };

                let register_c = instruction.output_register();
                registers_before.set(register_c, result);
            }
            Opcode::Eqir => {
                // eqir (equal immediate/register) sets register C to 1 if value A is equal to register B.
                // Otherwise, register C is set to 0.

                let value_a = instruction.input_a();

                let register_b = RegisterID::into_register_id(instruction.input_b());
                register_b.as_ref()?;
                let value_b = registers_before.get(register_b.unwrap());

                let result = if value_a == value_b { 1 } else { 0 };

                let register_c = instruction.output_register();
                registers_before.set(register_c, result);
            }
            Opcode::Eqri => {
                // eqri (equal register/immediate) sets register C to 1 if register A is equal to value B.
                // Otherwise, register C is set to 0.

                let register_a = RegisterID::into_register_id(instruction.input_a());
                register_a.as_ref()?;
                let value_a = registers_before.get(register_a.unwrap());

                let value_b = instruction.input_b();

                let result = if value_a == value_b { 1 } else { 0 };

                let register_c = instruction.output_register();
                registers_before.set(register_c, result);
            }
            Opcode::Eqrr => {
                // eqrr (equal register/register) sets register C to 1 if register A is equal to register B.
                // Otherwise, register C is set to 0.

                let register_a = RegisterID::into_register_id(instruction.input_a());
                register_a.as_ref()?;
                let value_a = registers_before.get(register_a.unwrap());

                let register_b = RegisterID::into_register_id(instruction.input_b());
                register_b.as_ref()?;
                let value_b = registers_before.get(register_b.unwrap());

                let result = if value_a == value_b { 1 } else { 0 };

                let register_c = instruction.output_register();
                registers_before.set(register_c, result);
            }
        }

        Some(registers_before)
    }
}

fn part_1(input_string: &str) -> OpCodeMap {
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
                .split(',')
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
                .map(|x| -> i32 { x.parse().unwrap() })
                .collect();

            OpcodeInstruction(
                arr[0],
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
                .split(',')
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

    let mut opcode_map: OpCodeMap = HashMap::new();
    let mut found_opcodes: HashSet<Opcode> = HashSet::new();
    let mut opcode_map_maybe: Vec<(i32, Vec<Opcode>)> = vec![];

    let remaining: Vec<(Registers, OpcodeInstruction, Registers)> = candidates
        .into_iter()
        .filter(|(before_register, opcode_instruction, after_register)| {
            let opcodes = Opcode::all_opcodes();

            assert!(opcodes.len() == 16);

            let matched_opcodes: Vec<Opcode> = opcodes
                .into_iter()
                .filter(|opcode| {
                    opcode.matches(
                        before_register.clone(),
                        opcode_instruction.clone(),
                        after_register.clone(),
                    )
                })
                .collect();

            if matched_opcodes.len() <= 1 {
                let opcode: Opcode = matched_opcodes.first().map(|x| (*x).clone()).unwrap();

                found_opcodes.insert(opcode.clone());
                opcode_map.insert(opcode_instruction.input_opcode_number(), opcode);
            }

            if matched_opcodes.len() > 1 {
                opcode_map_maybe.push((
                    opcode_instruction.input_opcode_number(),
                    matched_opcodes.clone(),
                ));
                // println!("{} {:?}", opcode_instruction.input_opcode_number(), matched_opcodes);
            }

            matched_opcodes.len() >= 3
        })
        .collect();

    println!("Part 1: {}", remaining.len());

    // construct opcode_map

    while !opcode_map_maybe.is_empty() {
        opcode_map_maybe = opcode_map_maybe
            .into_iter()
            .map(|(input_opcode_number, opcodes)| {
                let opcodes: Vec<Opcode> = opcodes
                    .into_iter()
                    .filter(|x| {
                        // filter out opcodes that are already found
                        !found_opcodes.contains(x)
                    })
                    .collect();

                if opcodes.len() == 1 {
                    let opcode: Opcode = opcodes.first().map(|x| (*x).clone()).unwrap();
                    found_opcodes.insert(opcode.clone());
                    opcode_map.insert(input_opcode_number, opcode);
                }

                (input_opcode_number, opcodes)
            })
            .filter(|(_input_opcode_number, opcodes)| opcodes.len() >= 2)
            .collect();
    }

    opcode_map
}

fn part_2(input_string: &str, opcode_map: OpCodeMap) {
    let mut inputs = input_string.trim().lines().peekable();

    // The registers start with the value 0.
    let mut registers = Registers(0, 0, 0, 0);

    while inputs.peek().is_some() {
        let input_line = inputs.next().unwrap().trim();

        // skip part 1 inputs
        if input_line.starts_with("Before: ") {
            // skip next two lines
            inputs.next();
            inputs.next();
            continue;
        }

        if input_line.is_empty() {
            continue;
        }

        let opcode_instruction = {
            let opcode_instruction_line = input_line;

            let arr: Vec<i32> = opcode_instruction_line
                .split_whitespace()
                .map(|x| x.trim())
                .map(|x| -> i32 { x.parse().unwrap() })
                .collect();

            OpcodeInstruction(
                arr[0],
                arr[1],
                arr[2],
                RegisterID::into_register_id(arr[3]).unwrap(),
            )
        };

        let opcode = opcode_map
            .get(&opcode_instruction.input_opcode_number())
            .unwrap();

        registers = opcode
            .execute(registers.clone(), opcode_instruction.clone())
            .unwrap();
    }

    println!("Part 2 registers: {:?}", registers);
    println!("Part 2: {}", registers.0);
}

fn main() {
    let input_string = include_str!("input.txt");

    let opcode_map = part_1(input_string);

    part_2(input_string, opcode_map);
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
            let instruction = OpcodeInstruction(99, 0, 1, RegisterID::Zero);
            let registers_after = Registers(3, 2, 3, 4);
            let result = opcode.matches(
                registers_before.clone(),
                instruction.clone(),
                registers_after,
            );

            assert_eq!(result, true);

            // invalid

            let registers_after = Registers(0, 2, 3, 4);
            let result = opcode.matches(registers_before, instruction, registers_after);

            assert_eq!(result, false);
        }
    }
}
