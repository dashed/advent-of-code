// https://adventofcode.com/2018/day/15

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

#[derive(Debug, PartialEq)]
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

enum Opcode {
    Addr,
}

impl Opcode {
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
        }

        return registers_before == registers_after;
    }
}

fn main() {
    let input_string = include_str!("input.txt");

    println!("{}", input_string);
}
