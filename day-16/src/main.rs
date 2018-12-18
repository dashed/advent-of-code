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
}

struct Registers(i32, i32, i32, i32);

struct OpcodeInstruction(
    i32,        /* input 1 */
    i32,        /* input 2 */
    RegisterID, /* output register */
);

impl OpcodeInstruction {
    fn output_register(&self) -> RegisterID {
        return self.2.clone();
    }
}

enum Opcode {
    Addr,
}

impl Opcode {
    fn process(&self, registers: Registers, instruction: OpcodeInstruction) -> Registers {
        match self {
            Opcode::Addr => {
                // addr (add register) stores into register C the result of adding register A and register B.
                let register_c = instruction.output_register();
            }
        }

        return registers;
    }
}

fn main() {
    let input_string = include_str!("input.txt");

    println!("{}", input_string);
}
