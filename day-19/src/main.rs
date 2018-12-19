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

struct Registers(i32, i32, i32, i32);

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

struct Program {
    // current value of the instruction pointer
    instruction_pointer: i32,
    // indicates the register that the instruction pointer is bound to
    instruction_pointer_bound: RegisterID,
    registers: Registers,
}

impl Program {
    // add code here
}

fn main() {
    let input_string = include_str!("input.txt");

    println!("{}", input_string);
}
