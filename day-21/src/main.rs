// https://adventofcode.com/2018/day/21

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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
    fn from_str(input: &str) -> Self {
        let input = input.to_lowercase();
        match input.as_ref() {
            "addr" => Opcode::Addr,
            "addi" => Opcode::Addi,
            "mulr" => Opcode::Mulr,
            "muli" => Opcode::Muli,
            "banr" => Opcode::Banr,
            "bani" => Opcode::Bani,
            "borr" => Opcode::Borr,
            "bori" => Opcode::Bori,
            "setr" => Opcode::Setr,
            "seti" => Opcode::Seti,
            "gtir" => Opcode::Gtir,
            "gtri" => Opcode::Gtri,
            "gtrr" => Opcode::Gtrr,
            "eqir" => Opcode::Eqir,
            "eqri" => Opcode::Eqri,
            "eqrr" => Opcode::Eqrr,
            _ => {
                unreachable!();
            }
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
                if register_a.is_none() {
                    return None;
                }

                let register_b = RegisterID::into_register_id(instruction.input_b());
                if register_b.is_none() {
                    return None;
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
                    return None;
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
                    return None;
                }

                let register_b = RegisterID::into_register_id(instruction.input_b());
                if register_b.is_none() {
                    return None;
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
                    return None;
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
                    return None;
                }

                let register_b = RegisterID::into_register_id(instruction.input_b());
                if register_b.is_none() {
                    return None;
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
                    return None;
                }

                let value_a = registers_before.get(register_a.unwrap());
                let value_b = instruction.input_b();

                let result = value_a & value_b;

                let register_c = instruction.output_register();
                registers_before.set(register_c, result);
            }
            Opcode::Borr => {
                // borr (bitwise OR register) stores into register C the result of the bitwise OR of register A and register B.

                let register_a = RegisterID::into_register_id(instruction.input_a());
                if register_a.is_none() {
                    return None;
                }

                let register_b = RegisterID::into_register_id(instruction.input_b());
                if register_b.is_none() {
                    return None;
                }

                let value_a = registers_before.get(register_a.unwrap());
                let value_b = registers_before.get(register_b.unwrap());

                let register_c = instruction.output_register();

                let result = value_a | value_b;
                registers_before.set(register_c, result);
            }
            Opcode::Bori => {
                // bori (bitwise OR immediate) stores into register C the result of the bitwise OR of register A and value B.

                let register_a = RegisterID::into_register_id(instruction.input_a());
                if register_a.is_none() {
                    return None;
                }

                let value_a = registers_before.get(register_a.unwrap());
                let value_b = instruction.input_b();

                let result = value_a | value_b;

                let register_c = instruction.output_register();
                registers_before.set(register_c, result);
            }
            Opcode::Setr => {
                // setr (set register) copies the contents of register A into register C. (Input B is ignored.)

                let register_a = RegisterID::into_register_id(instruction.input_a());
                if register_a.is_none() {
                    return None;
                }

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
                if register_b.is_none() {
                    return None;
                }
                let value_b = registers_before.get(register_b.unwrap());

                let result = if value_a > value_b { 1 } else { 0 };

                let register_c = instruction.output_register();
                registers_before.set(register_c, result);
            }
            Opcode::Gtri => {
                // gtri (greater-than register/immediate) sets register C to 1 if register A is greater than value B.
                // Otherwise, register C is set to 0.

                let register_a = RegisterID::into_register_id(instruction.input_a());
                if register_a.is_none() {
                    return None;
                }
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
                if register_a.is_none() {
                    return None;
                }
                let value_a = registers_before.get(register_a.unwrap());

                let register_b = RegisterID::into_register_id(instruction.input_b());
                if register_b.is_none() {
                    return None;
                }
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
                if register_b.is_none() {
                    return None;
                }
                let value_b = registers_before.get(register_b.unwrap());

                let result = if value_a == value_b { 1 } else { 0 };

                let register_c = instruction.output_register();
                registers_before.set(register_c, result);
            }
            Opcode::Eqri => {
                // eqri (equal register/immediate) sets register C to 1 if register A is equal to value B.
                // Otherwise, register C is set to 0.

                let register_a = RegisterID::into_register_id(instruction.input_a());
                if register_a.is_none() {
                    return None;
                }
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
                if register_a.is_none() {
                    return None;
                }
                let value_a = registers_before.get(register_a.unwrap());

                let register_b = RegisterID::into_register_id(instruction.input_b());
                if register_b.is_none() {
                    return None;
                }
                let value_b = registers_before.get(register_b.unwrap());

                let result = if value_a == value_b { 1 } else { 0 };

                let register_c = instruction.output_register();
                registers_before.set(register_c, result);
            }
        }

        return Some(registers_before);
    }
}

#[derive(Debug, Clone)]
struct OpcodeInstruction(
    Opcode,
    i32,        /* input A */
    i32,        /* input B */
    RegisterID, /* output register */
);

impl OpcodeInstruction {
    fn output_register(&self) -> RegisterID {
        return self.3.clone();
    }

    fn input_a(&self) -> i32 {
        return self.1;
    }

    fn input_b(&self) -> i32 {
        return self.2;
    }

    fn opcode(&self) -> Opcode {
        return self.0.clone();
    }
}

enum Status {
    Halted,
    NotHalted,
}

#[derive(Debug, Clone)]
struct Program {
    // current value of the instruction pointer
    instruction_pointer: i32,
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

    fn fork(&self) -> Self {
        Program {
            // The instruction pointer starts at 0.
            instruction_pointer: 0,
            instruction_pointer_bound: self.instruction_pointer_bound.clone(),
            registers: Registers::new(),
            instructions: self.instructions.clone(),
        }
    }

    fn execute_instruction(&mut self) -> Status {
        // get next instruction
        let instruction = self.instructions.get(self.instruction_pointer as usize);

        if instruction.is_none() {
            // If the instruction pointer ever causes the device to attempt to load
            // an instruction outside the instructions defined in the program, the program instead immediately halts.
            return Status::Halted;
        }

        let instruction: &OpcodeInstruction = instruction.unwrap();

        // write the value in instruction pointer to the bound register
        self.registers.set(
            self.instruction_pointer_bound.clone(),
            self.instruction_pointer,
        );

        // execute instruction
        let opcode = instruction.opcode();
        self.registers = opcode
            .execute(self.registers.clone(), instruction.clone())
            .unwrap();

        // write bound register back to the instruction poiinter
        self.instruction_pointer = self.registers.get(self.instruction_pointer_bound.clone());

        // after the instruction has executed, add one to the instruction pointer
        self.instruction_pointer += 1;

        return Status::NotHalted;
    }
}

fn parse_input(input_string: &str) -> Program {
    let mut input_iter = input_string.trim().lines().map(|s| s.trim());

    let instruction_pointer_bound: RegisterID = {
        let mut iter = input_iter.next().unwrap().split_whitespace();
        iter.next();
        let register_num = iter
            .next()
            .map(|s| s.trim().parse::<i32>().unwrap())
            .unwrap();
        RegisterID::into_register_id(register_num).unwrap()
    };

    let mut instructions = vec![];

    while let Some(opcode_instruction_line) = input_iter.next() {
        let opcode_instruction = {
            let mut iter = opcode_instruction_line.split_whitespace().map(|x| x.trim());

            let opcode_str = iter.next().unwrap();

            let arr: Vec<i32> = iter
                .map(|x| -> i32 {
                    return x.parse().unwrap();
                })
                .collect();

            OpcodeInstruction(
                Opcode::from_str(opcode_str),
                arr[0],
                arr[1],
                RegisterID::into_register_id(arr[2]).unwrap(),
            )
        };

        instructions.push(opcode_instruction);
    }

    let program = Program::new(instruction_pointer_bound.clone(), instructions.clone());

    return program;
}

fn has_halted(mut program: Program, num_of_instructions: i32) -> bool {
    // has the program halted within num_of_instructions?

    let mut current_instructions = 1;

    while current_instructions <= num_of_instructions {
        let result = program.execute_instruction();
        match result {
            Status::Halted => {
                return true;
            }
            _ => {}
        }

        current_instructions += 1;
    }

    return false;
}

/*
#ip 1
seti 123 0 5            0 reg[5] = 123
bani 5 456 5            1 reg[5] = reg[5] & 456.        note: 123 & 456 is 72
eqri 5 72 5             2 reg[5] = reg[5] == 72
addr 5 1 1              3 reg[1] = reg[5] + reg[1]
seti 0 0 1              4 reg[1] = 0
seti 0 7 5              5 reg[5] = 0
bori 5 65536 4          6 reg[4] = reg[5] | 65536
seti 13159625 6 5       7 reg[5] = 13159625
bani 4 255 3            8 reg[3] = reg[4] & 255
addr 5 3 5              9 reg[5] = reg[5] + reg[3]
bani 5 16777215 5      10 reg[5] = reg[5] & 16777215
muli 5 65899 5         11 reg[5] = reg[5] * 65899
bani 5 16777215 5      13 reg[5] = reg[5] & 16777215
gtir 256 4 3           14 reg[3] = 256 > reg[4]
addr 3 1 1             15 reg[1] = reg[3] + reg[1]
addi 1 1 1             16 reg[1] = reg[1] + reg[1]
seti 27 9 1            17 reg[1] = 27
seti 0 0 3             18 reg[3] = 0
addi 3 1 2             19 reg[2] = reg[3] + 1
muli 2 256 2           20 reg[2] = reg[2] * 256
gtrr 2 4 2             21 reg[2] = reg[2] > reg[4]
addr 2 1 1             22 reg[1] = reg[2] + reg[1]
addi 1 1 1             23 reg[1] = reg[1] + 1
seti 25 0 1            24 reg[1] = 25
addi 3 1 3             25 reg[3] = reg[3] + 1
seti 17 4 1            26 reg[1] = 17
setr 3 3 4             27 reg[4] = reg[3]
seti 7 5 1             28 reg[1] = 7
eqrr 5 0 3             29 reg[3] = reg[5] == reg[0]
addr 3 1 1             30 reg[1] = reg[3] + reg[1]
seti 5 6 1             31 reg[1] = 5
*/

fn part_1(mut program: Program) {
    // for input in 0..300000 {
    //     let mut program = program.clone();
    //     program.registers.set(RegisterID::Zero, input);

    //     let halted = has_halted(program, 10000);

    //     if halted {
    //         println!("Part 1: {}", input);
    //         return;
    //     }
    // }

    // println!(":(");

    while program.instruction_pointer <= 7 {
        println!("{}: {:?}", program.instruction_pointer, program.registers);
        let result = program.execute_instruction();
        match result {
            Status::Halted => {
                break;
            }
            _ => {}
        }

        println!("{}: {:?}", program.instruction_pointer, program.registers);

        println!("--------");
    }
}

fn main() {
    let input_string = include_str!("input.txt");

    let program = parse_input(input_string);

    part_1(program.clone());
}