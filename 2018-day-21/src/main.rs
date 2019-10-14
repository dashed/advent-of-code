// https://adventofcode.com/2018/day/21

// imports

use std::collections::HashMap;

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

    #[allow(dead_code)]
    fn print_next_intstruction(&self) {
        let instruction = self.instructions.get(self.instruction_pointer as usize);
        println!("{:?}", instruction);
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

fn compiled_program(reg_0: i32) {
    let mut num_of_instructions_executed: i128 = 0;

    // registers
    let reg_0 = reg_0;
    // let mut reg_1 = 0;
    let mut reg_2;
    let mut reg_3;
    let mut reg_4;
    let mut reg_5;

    // start of program

    // seti 123 0 5
    reg_5 = 123;
    num_of_instructions_executed += 1;

    loop {
        // bani 5 456 5
        reg_5 = reg_5 & 456;
        num_of_instructions_executed += 1;

        // eqri 5 72 5
        num_of_instructions_executed += 1;
        reg_5 = if reg_5 == 72 { 1 } else { 0 };

        // addr 5 1 1
        num_of_instructions_executed += 1;
        if reg_5 == 1 {
            break;
        } else {
            // seti 0 0 1
            num_of_instructions_executed += 1;
            continue;
        }
    }

    // seti 0 7 5
    reg_5 = 0;
    num_of_instructions_executed += 1;

    let mut lookup: HashMap<i32, i128> = HashMap::new();

    loop {
        // loop B

        // bori 5 65536 4
        reg_4 = reg_5 | 65536;
        num_of_instructions_executed += 1;

        // seti 13159625 6 5
        reg_5 = 13159625;

        num_of_instructions_executed += 1;

        loop {
            // loop C

            // bani 4 255 3
            reg_3 = reg_4 & 255;

            // addr 5 3 5
            reg_5 = reg_5 + reg_3;

            // bani 5 16777215 5
            reg_5 = reg_5 & 16777215;

            // muli 5 65899 5
            reg_5 = reg_5 * 65899;

            // bani 5 16777215 5
            reg_5 = reg_5 & 16777215;

            num_of_instructions_executed += 5;

            // gtir 256 4 3
            reg_3 = if 256 > reg_4 { 1 } else { 0 };
            num_of_instructions_executed += 1;

            // addr 3 1 1
            num_of_instructions_executed += 1;
            if reg_3 == 1 {
                // seti 27 9 1
                // break out of loop C
                num_of_instructions_executed += 1;
                break;
            } else {
                // addi 1 1 1
                // continue loop C
                num_of_instructions_executed += 1;
            }

            // seti 0 0 3
            reg_3 = 0;
            num_of_instructions_executed += 1;

            loop {
                // loop A

                // while loop guard

                // addi 3 1 2
                reg_2 = reg_3 + 1;
                num_of_instructions_executed += 1;

                // muli 2 256 2
                reg_2 = reg_2 * 256;
                num_of_instructions_executed += 1;

                // gtrr 2 4 2
                reg_2 = if reg_2 > reg_4 { 1 } else { 0 };
                num_of_instructions_executed += 1;

                // addr 2 1 1
                num_of_instructions_executed += 1;
                if reg_2 == 1 {
                    // break out of loop A

                    // seti 25 0 1
                    num_of_instructions_executed += 1;
                    break;
                } else {
                    // addi 1 1 1
                    num_of_instructions_executed += 1;
                    // continue loop A
                }

                // loop body
                // addi 3 1 3
                reg_3 = reg_3 + 1;
                num_of_instructions_executed += 1;

                // rerun loop A
                // seti 17 4 1
                num_of_instructions_executed += 1;
            }

            // setr 3 3 4
            reg_4 = reg_3;
            num_of_instructions_executed += 1;

            // seti 7 5 1
            // continue loop C
            num_of_instructions_executed += 1;
        }

        // do-while loop guard for loop B:

        // eqrr 5 0 3
        reg_3 = if reg_5 == reg_0 { 1 } else { 0 };
        num_of_instructions_executed += 1;

        match lookup.get(&reg_5) {
            None => {
                lookup.insert(reg_5, num_of_instructions_executed);
                // println!("reg_5 = {}; num_of_instructions_executed = {}", reg_5, num_of_instructions_executed);
            }
            Some(_) => {
                reg_3 = 1;
            }
        }

        // addr 3 1 1
        num_of_instructions_executed += 1;
        if reg_3 == 1 {
            // program exit

            // technically lp=30 will be attempted
            num_of_instructions_executed += 1;

            break;
        } else {
            // seti 5 6 1
            num_of_instructions_executed += 1;
            // continue loop
        }
    }

    // println!("{}", reg_5);
    println!(
        "num_of_instructions_executed: {}",
        num_of_instructions_executed
    );

    let (best_reg_0_value, min_num_of_instructions) = lookup
        .iter()
        .min_by_key(|item: &(&i32, &i128)| -> i128 {
            let (_key, value) = *item;
            return *value;
        })
        .unwrap();

    println!(
        "reg_0 should be {} which halts after executing at minimum {} instructions",
        best_reg_0_value, min_num_of_instructions
    );

    let (lol, max_num_of_instructions) = lookup
        .iter()
        .max_by_key(|item: &(&i32, &i128)| -> i128 {
            let (_key, value) = *item;
            return *value;
        })
        .unwrap();

    println!(
        "reg_0 should be {} which halts after executing at  maximum {} instructions",
        lol, max_num_of_instructions
    );

    // println!(
    //     "Registers Registers({}, {}, {}, {}, {}, {})",
    //     reg_0, "_", reg_2, reg_3, reg_4, reg_5
    // );
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

-----------

#ip 1
seti 123 0 5            0 reg[5] = 123
bani 5 456 5            1 reg[5] = reg[5] & 456.        note: 123 & 456 is 72
eqri 5 72 5             2 reg[5] = reg[5] == 72         condition: 123 & 456 == 72
addr 5 1 1              3 reg[1] = reg[5] + reg[1]      if condition == 1, start from lp=4
                                                        otherwise start from lp=3

seti 0 0 1              4 reg[1] = 0                    start from lp=0


seti 0 7 5              5 reg[5] = 0                    reg_5 = 0

---------
loop B start:

bori 5 65536 4          6 reg[4] = reg[5] | 65536

seti 13159625 6 5       7 reg[5] = 13159625

---------
loop C start:

bani 4 255 3            8 reg[3] = reg[4] & 255
addr 5 3 5              9 reg[5] = reg[5] + reg[3]
bani 5 16777215 5      10 reg[5] = reg[5] & 16777215
muli 5 65899 5         11 reg[5] = reg[5] * 65899
bani 5 16777215 5      12 reg[5] = reg[5] & 16777215    (((13159625 + ((reg_5 | 65536) & 255)) & 16777215) * 65899) & 16777215

gtir 256 4 3           13 reg[3] = 256 > reg[4]                 condition: reg_3 = 256 > (reg_5 | 65536)
addr 3 1 1             14 reg[1] = reg[3] + reg[1]              if condition == 1, start from lp=15,
                                                                otherwise, start from lp=14
addi 1 1 1             15 reg[1] = reg[1] + 1.                  if body: start from lp=16 (execute loop A sequence)

seti 27 9 1            16 reg[1] = 27                           start from lp=27 (break loop C)



seti 0 0 3             17 reg[3] = 0                            reg_3 = 0

---------
loop A start:

                                                                while loop guard for loop A:
addi 3 1 2             18 reg[2] = reg[3] + 1                   part of loop exit condition
muli 2 256 2           19 reg[2] = reg[2] * 256                 part of loop exit condition
gtrr 2 4 2             20 reg[2] = reg[2] > reg[4]              loop exit condition := (reg_3 + 1) * 256 > reg_4


addr 2 1 1             21 reg[1] = reg[2] + reg[1]              if exit condition == 1, start from lp=22 (exit),
                                                                otherwise start from lp=21 (re-run loop)

addi 1 1 1             22 reg[1] = reg[1] + 1                   continue loop: start from lp=23

seti 25 0 1            23 reg[1] = 25                           exit loop; start from lp=25

addi 3 1 3             24 reg[3] = reg[3] + 1                   loop body: reg_3 = reg_3 + 1

seti 17 4 1            25 reg[1] = 17                           re-run loop: start from lp=17

loop A end
---------

setr 3 3 4             26 reg[4] = reg[3]                       set temp variable
seti 7 5 1             27 reg[1] = 7                            start from lp=7 (continue loop C)

loop C end
---------

do-while loop guard for loop B:
eqrr 5 0 3             28 reg[3] = reg[5] == reg[0]             condition: reg_5 == reg_0
addr 3 1 1             29 reg[1] = reg[3] + reg[1].             if condition == 1, start at lp=30 (program exit),
                                                                otherwise, start from lp=30 (do-while loop re-run)

seti 5 6 1             30 reg[1] = 5                            start from lp=5

loop B end
---------

*/

fn part_1(mut program: Program, reg_0: i32) {
    program.registers.set(RegisterID::Zero, reg_0);

    let mut num_of_instructions_executed = 0;

    loop {
        if program.instruction_pointer == 18 {
            let mut reg_3 = program.registers.get(RegisterID::Three);
            let reg_4 = program.registers.get(RegisterID::Four);

            loop {
                // addi 3 1 2
                // muli 2 256 2
                // gtrr 2 4 2
                // addr 2 1 1
                num_of_instructions_executed += 4;
                if (reg_3 + 1) * 256 > reg_4 {
                    // seti 25 0 1
                    num_of_instructions_executed += 1;
                    break;
                }
                // addi 1 1 1
                // continue loop body
                num_of_instructions_executed += 1;

                // addi 3 1 3
                num_of_instructions_executed += 1;
                reg_3 += 1;

                // seti 17 4 1
                num_of_instructions_executed += 1;
                // re-run loop
            }

            program.registers.set(RegisterID::Three, reg_3);
            program.registers.set(RegisterID::Two, 1);

            program.instruction_pointer = 26;

            continue;
        }

        let result = program.execute_instruction();
        num_of_instructions_executed += 1;
        match result {
            Status::Halted => {
                break;
            }
            _ => {}
        }
    }

    println!("-----");

    println!(
        "Verification by executing program. num_of_instructions_executed: {}",
        num_of_instructions_executed
    );
    // println!("Registers {:?}", program.registers);
}

fn main() {
    compiled_program(0);

    let input_string = include_str!("input.txt");
    let program = parse_input(input_string);
    part_1(program.clone(), 3941014);
}
