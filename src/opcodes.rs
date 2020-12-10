use std::collections::HashMap;

const INSTRUCTIONS: [(&'static str, &'static InstructionParser, Opcodes); 40] = [
    (
        "loadliteral",
        &InstructionParser::R1Lit,
        Opcodes::LoadLiteral,
    ),
    ("print", &InstructionParser::R1, Opcodes::Print),
    ("halt", &InstructionParser::R0, Opcodes::Halt),
    ("goto", &InstructionParser::R0I24, Opcodes::Goto),
    ("!", &InstructionParser::R2, Opcodes::Not),
    ("!=", &InstructionParser::R2, Opcodes::NotEqual),
    ("mov", &InstructionParser::R2, Opcodes::Mov),
    ("if", &InstructionParser::R1, Opcodes::If),
    ("+", &InstructionParser::R3, Opcodes::Add),
    ("-", &InstructionParser::R3, Opcodes::Subtract),
    ("*", &InstructionParser::R3, Opcodes::Multiply),
    ("/", &InstructionParser::R3, Opcodes::Divide),
    ("=", &InstructionParser::R3, Opcodes::Equal),
    ("check", &InstructionParser::R1Lit, Opcodes::Check),
    ("expect", &InstructionParser::R1Lit, Opcodes::Expect),
    ("check-assert", &InstructionParser::R1Lit, Opcodes::Assert),
    ("setglobal", &InstructionParser::R1Lit, Opcodes::SetGlobal),
    ("getglobal", &InstructionParser::R1Lit, Opcodes::GetGlobal),
    ("number?", &InstructionParser::R2, Opcodes::IsNumber),
    ("symbol?", &InstructionParser::R2, Opcodes::IsSymbol),
    ("boolean?", &InstructionParser::R2, Opcodes::IsBoolean),
    ("null?", &InstructionParser::R2, Opcodes::IsNull),
    ("nil?", &InstructionParser::R2, Opcodes::IsNil),
    (">", &InstructionParser::R3, Opcodes::Greater),
    ("<", &InstructionParser::R3, Opcodes::Less),
    ("<=", &InstructionParser::R3, Opcodes::LessEq),
    ("call", &InstructionParser::R3, Opcodes::Call),
    ("tailcall", &InstructionParser::R2, Opcodes::TailCall),
    ("return", &InstructionParser::R1, Opcodes::Return),
    ("cons", &InstructionParser::R3, Opcodes::Cons),
    ("car", &InstructionParser::R2, Opcodes::Car),
    ("cdr", &InstructionParser::R2, Opcodes::Cdr),
    ("mkclosure", &InstructionParser::R3, Opcodes::MakeClosure),
    ("getclslot", &InstructionParser::R3, Opcodes::GetClSlot),
    ("setclslot", &InstructionParser::R3, Opcodes::SetClSlot),
    ("set-car!", &InstructionParser::R2, Opcodes::SetCar),
    ("set-cdr!", &InstructionParser::R2, Opcodes::SetCdr),
    ("idiv", &InstructionParser::R3, Opcodes::IDiv),
    ("pair?", &InstructionParser::R2, Opcodes::Pair),
    ("error", &InstructionParser::R1, Opcodes::Error),
];

#[derive(Copy, Clone)]
pub enum InstructionParser {
    R3,
    R2,
    R1,
    R0,
    R1Lit,
    R0I24,
}
#[derive(Debug, Clone, Copy, Hash)]
pub enum Opcodes {
    Add,
    LoadLiteral,
    Print,
    Halt,
    Goto,
    Not,
    Mov,
    If,
    Subtract,
    Multiply,
    Divide,
    Equal,
    Check,
    Expect,
    SetGlobal,
    GetGlobal,
    IsSymbol,
    IsBoolean,
    Assert,
    IsNil,
    IsNull,
    IsNumber,
    Greater,
    Less,
    LessEq,
    Return,
    Call,
    TailCall,
    Cons,
    Car,
    Cdr,
    MakeClosure,
    SetClSlot,
    GetClSlot,
    SetCar,
    SetCdr,
    NotEqual,
    IDiv,
    Pair,
    Error,
}
#[derive(Debug, Copy, Clone, Hash)]
pub struct Instruction {
    pub opcode: Opcodes,
    pub r_x: usize,
    pub r_y: usize,
    pub r_z: usize,
    pub slot: usize,
    pub goto: i32,
}

impl Instruction {
    pub fn eru16(opcode: Opcodes, slot: usize, reg: usize) -> Self {
        Instruction {
            opcode,
            r_x: reg,
            r_y: 0,
            r_z: 0,
            slot,
            goto: 0,
        }
    }
}

pub fn get_parsers() -> HashMap<String, (InstructionParser, Opcodes)> {
    let mut map = HashMap::new();
    for (opcode, ptype, op) in INSTRUCTIONS.iter() {
        map.insert(opcode.to_string(), (**ptype, *op));
    }
    map
}
