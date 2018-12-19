// https://adventofcode.com/2018/day/19

// code

#[derive(Debug, Clone)]
enum RegisterID {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
}

impl RegisterID {
    fn into_register_id(input: i32) -> Option<RegisterID> {
        match input {
            0 => Some(RegisterID::Zero),
            1 => Some(RegisterID::One),
            2 => Some(RegisterID::Two),
            3 => Some(RegisterID::Three),
            4 => Some(RegisterID::Four),
            5 => Some(RegisterID::Five),
            _ => None,
        }
    }
}

struct Registers(i32, i32, i32, i32, i32, i32);

impl Registers {
    fn new() -> Self {
        return Registers(0, 0, 0, 0, 0, 0);
    }

    fn get(&self, register_id: RegisterID) -> i32 {
        match register_id {
            RegisterID::Zero => self.0,
            RegisterID::One => self.1,
            RegisterID::Two => self.2,
            RegisterID::Three => self.3,
            RegisterID::Four => self.4,
            RegisterID::Five => self.5,
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
            RegisterID::Four => {
                self.4 = value;
            }

            RegisterID::Five => {
                self.5 = value;
            }
        }
    }
}

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

struct OpcodeInstruction(
    Opcode,
    i32,        /* input A */
    i32,        /* input B */
    RegisterID, /* output register */
);

enum Status {
    Halted,
    NotHalted,
}

struct Program {
    // current value of the instruction pointer
    instruction_pointer: usize,
    // indicates the register that the instruction pointer is bound to
    instruction_pointer_bound: RegisterID,
    registers: Registers,

    instructions: Vec<OpcodeInstruction>,
}

impl Program {
    fn new(instruction_pointer_bound: RegisterID, instructions: Vec<OpcodeInstruction>) -> Self {
        Program {
            // The instruction pointer starts at 0.
            instruction_pointer: 0,
            instruction_pointer_bound,
            registers: Registers::new(),
            instructions,
        }
    }

    fn run_program(&mut self) {
        loop {
            let result = self.execute_instruction();
            match result {
                Status::Halted => {
                    break;
                }
                _ => {}
            }
        }
    }

    fn execute_instruction(&mut self) -> Status {
        // get next instruction
        let instruction = self.instructions.get(self.instruction_pointer);

        if instruction.is_none() {
            // If the instruction pointer ever causes the device to attempt to load
            // an instruction outside the instructions defined in the program, the program instead immediately halts.
            return Status::Halted;
        }

        // TODO: write the value in instruction pointer to the bound register

        // TODO: execute instruction

        // TODO: write bound register back to the instruction poiinter

        // after the instruction has executed, add one to the instruction pointer
        self.instruction_pointer += 1;

        return Status::NotHalted;
    }
}

fn main() {
    let input_string = include_str!("input.txt");

    println!("{}", input_string);
}
