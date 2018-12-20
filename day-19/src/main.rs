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

        use std::thread;
        use std::time::Duration;
        thread::sleep(Duration::from_millis(100));
        println!("{:?}", self.instruction_pointer);

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

fn main() {
    let input_string = include_str!("input.txt");

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

    let mut program = Program::new(instruction_pointer_bound.clone(), instructions.clone());

    program.run_program();

    println!("Part 1: {}", program.registers.get(RegisterID::Zero));

    // A new background process immediately spins up in its place.
    // It appears identical, but on closer inspection, you notice that this time,
    // register 0 started with the value 1.

    let mut other_program = Program::new(instruction_pointer_bound, instructions);

    other_program.registers.set(RegisterID::Zero, 1);
    other_program.run_program();

    println!("Part 2: {}", other_program.registers.get(RegisterID::Zero));

    // part 2 spends quite a lot of time on these instructions

    // (BEGIN do while loop body)
    //
    // instruction 3:
    //
    // OpcodeInstruction(Mulr, 3, 1, Two)
    // reg[2] = reg[3] * reg[1]
    //
    // part of if block guard

    // instruction 4:
    //
    // OpcodeInstruction(Eqrr, 2, 5, Two)
    //
    // reg[2] = reg[2] == reg[5]
    //
    // if block guard result

    // instruction 5:
    //
    // OpcodeInstruction(Addr, 2, 4, Four)
    //
    // reg[4] = reg[2] + reg[4]
    //
    // if reg[2] == 0, go to instruction 6 (else block)
    // if reg[2] == 1, go to instruction 7 (if block)

    // instruction 6:
    //
    // OpcodeInstruction(Addi, 4, 1, Four)
    //
    // reg[4] = reg[4] + 1
    //
    // go to instruction 8
    //
    // NOTE: notice that there is no else block; the if-block was skipped

    // instruction 7:
    //
    // addr 3 0 0
    //
    // reg[0] = reg[3] + reg[0]
    //
    // if block

    // instruction 8:
    //
    // OpcodeInstruction(Addi, 1, 1, One)
    //
    // reg[1] = reg[1] + 1

    // (END do while loop body)

    // instruction 9:
    //
    // OpcodeInstruction(Gtrr, 1, 5, Two)
    //
    // reg[2] = reg[1] > reg[5]
    //
    // part of do while loop guard

    // instruction 10:
    //
    // OpcodeInstruction(Addr, 4, 2, Four)
    //
    // reg[4] = reg[4] + reg[2]
    //
    // if reg[2] == 0, go to instruction 11, (this is like a not operation on the while-loop guard)
    // otherwise if reg[2] == 1, go to instruction 12 (exit the loop)

    // instruction 11:
    //
    // OpcodeInstruction(Seti, 2, 9, Four)
    //
    // reg[4] = 2
    //
    // go to instruction 3 (i.e. run do-while loop body again)
    //

    // instruction 12:
    //
    // addi 3 1 3
    //
    // reg[3] = reg[3] + 1

    /*

    naive interpretation:

    do {

        if(reg[3] * reg[1] == reg[5]) {
            reg[0] = reg[3] + reg[0]
        }

        reg[1] = reg[1] + 1

    } while ( !( reg[1] > reg[5] ) )

    */
}
