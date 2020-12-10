use core::panic;
use either::Either;
use nom::{self, character::complete::digit1, IResult};
use std::{collections::HashMap, iter::FromIterator};
use std::{fs::File, io::*};
use value::value::VMFunction;

use crate::{
    opcodes::{Instruction, InstructionParser, Opcodes},
    value::{self, value::Val},
    vmstate::VMState,
};

pub fn load_modules(
    x: Either<Stdin, File>,
    parser_map: &HashMap<String, (InstructionParser, Opcodes)>,
    vm: &mut VMState,
) -> VMFunction {
    let mut buf = String::new();
    match x {
        Either::Left(mut stdin) => stdin
            .read_to_string(&mut buf)
            .expect("Failed to read from stdin"),
        Either::Right(mut file) => file.read_to_string(&mut buf).expect("Failed to read file"),
    };

    match parse_modules(&buf, parser_map, vm) {
        Ok((_, foo)) => foo,
        Err(e) => panic!("Wrong input {}", e),
    }
}

fn parse_modules<'a>(
    input: &'a str,
    parser_map: &HashMap<String, (InstructionParser, Opcodes)>,
    vm: &mut VMState,
) -> IResult<&'a str, VMFunction> {
    let r: IResult<&str, &str> = nom::bytes::complete::tag(".load module")(input);
    if let Ok((rest, _)) = r {
        let (rest, _) = nom::character::complete::multispace0(rest)?;
        let (rest, str_size) = digit1(rest)?;
        let size = str_size.parse().unwrap();
        parse_module(0, size, rest, parser_map, vm)
    } else {
        panic!("failed parsing")
    }
}

fn parse_module<'a>(
    arity: i32,
    count: i32,
    rest: &'a str,
    parser_map: &HashMap<String, (InstructionParser, Opcodes)>,
    vm: &mut VMState,
) -> IResult<&'a str, VMFunction> {
    let mut vm_function = VMFunction {
        size: count,
        arity: arity,
        nregs: 0,
        instructions: Vec::new(),
    };
    let mut stream = rest;
    for _ in 0..count {
        let (rest, _) = nom::character::complete::multispace0(stream)?;
        let (rest, name) = nom::bytes::complete::is_not(" \t\n")(rest)?;
        if name == ".load" {
            let (rest, _) = nom::character::complete::multispace0(rest)?;
            let (rest, reg) = nom::character::complete::digit1(rest)?;
            let (rest, _fun_name) = nom::bytes::complete::tag(" function ")(rest)?;
            let (rest, fun_arity) = nom::character::complete::digit1(rest)?;
            let (rest, _) = nom::character::complete::multispace0(rest)?;
            let (rest, fun_length) = nom::character::complete::digit1(rest)?;
            let (rest, func) = parse_module(
                fun_arity.parse().unwrap(),
                fun_length.parse().unwrap(),
                rest,
                parser_map,
                vm,
            )?;

            let slot = vm.literal_slot(Val::VMFunction(func));
            let i = Instruction::eru16(Opcodes::LoadLiteral, slot, reg.parse().unwrap());
            vm_function.instructions.push(i);
            stream = rest;
        } else {
            let (rest, instruction) = parse_instruction(stream, parser_map, vm)?;
            stream = rest;
            vm_function.instructions.push(instruction);
        }
    }
    Ok((stream, vm_function))
}

fn parse_instruction<'a>(
    s: &'a str,
    parser_map: &HashMap<String, (InstructionParser, Opcodes)>,
    vm: &mut VMState,
) -> IResult<&'a str, Instruction> {
    let (rest, _) = nom::character::complete::multispace0(s)?;
    let (rest, instruction_name) =
        nom::multi::many1(nom::character::complete::none_of(" \t\n\r"))(rest)?;
    let ins = String::from_iter(instruction_name);
    let (parser, opcode) = parser_map
        .get(&ins)
        .expect(&format!("Unknown instruction: {}", ins));
    let (rest, instruction) = match parser {
        InstructionParser::R3 => parse_r3(opcode, rest),
        InstructionParser::R2 => parse_r2(opcode, rest),
        InstructionParser::R1 => parse_r1(opcode, rest),
        InstructionParser::R0 => parse_r0(opcode, rest),
        InstructionParser::R1Lit => parse_r1lit(vm, opcode, rest),
        InstructionParser::R0I24 => parse_r0i24(opcode, rest),
    }?;
    Ok((rest, instruction))
}

fn parse_r0i24<'a>(opcode: &Opcodes, rest: &'a str) -> IResult<&'a str, Instruction> {
    let (rest, _) = nom::character::complete::multispace0(rest)?;
    let (rest, v) = nom::sequence::pair(
        nom::combinator::opt(nom::bytes::complete::tag("-")),
        nom::character::complete::digit1,
    )(rest)?;
    let num: i32 = if !v.0.is_none() {
        v.1.parse::<i32>().unwrap() * -1
    } else {
        v.1.parse().unwrap()
    };
    Ok((
        rest,
        Instruction {
            opcode: opcode.clone(),
            r_x: 0,
            r_y: 0,
            r_z: 0,
            slot: 0,
            goto: num,
        },
    ))
}

fn parse_r2<'a>(opcode: &Opcodes, rest: &'a str) -> IResult<&'a str, Instruction> {
    let (rest, _) = nom::character::complete::multispace0(rest)?;
    let (rest, r_x) = nom::character::complete::digit1(rest)?;
    let (rest, _) = nom::character::complete::multispace0(rest)?;
    let (rest, r_y) = nom::character::complete::digit1(rest)?;
    Ok((
        rest,
        Instruction {
            opcode: opcode.clone(),
            r_x: r_x.parse().unwrap(),
            r_y: r_y.parse().unwrap(),
            r_z: 0,
            slot: 0,
            goto: 0,
        },
    ))
}

fn parse_r0<'a>(opcode: &Opcodes, rest: &'a str) -> IResult<&'a str, Instruction> {
    Ok((
        rest,
        Instruction {
            r_x: 0,
            r_y: 0,
            r_z: 0,
            opcode: opcode.clone(),
            slot: 0,
            goto: 0,
        },
    ))
}

fn parse_r1lit<'a>(
    vm: &mut VMState,
    opcode: &Opcodes,
    rest: &'a str,
) -> IResult<&'a str, Instruction> {
    let (rest, _) = nom::character::complete::multispace0(rest)?;
    let (rest, reg) = nom::character::complete::digit1(rest)?;
    let (rest, _) = nom::character::complete::multispace0(rest)?;
    let (_, first_token) = nom::character::complete::anychar(rest)?;
    match first_token.is_ascii_digit() {
        true => {
            let (rest, num) = nom::character::complete::digit1(rest)?;
            let slot = vm.literal_slot(Val::to_num(num.parse().unwrap()));
            Ok((
                rest,
                Instruction {
                    r_x: reg.parse().unwrap(),
                    r_y: 0,
                    r_z: 0,
                    opcode: opcode.clone(),
                    slot,
                    goto: 0,
                },
            ))
        }
        false => parse_complex_lit(vm, opcode, rest, reg.parse().unwrap()),
    }
}

fn parse_complex_lit<'a>(
    vm: &mut VMState,
    opcode: &Opcodes,
    rest: &'a str,
    reg: usize,
) -> IResult<&'a str, Instruction> {
    let (rest, t) = nom::character::complete::alphanumeric1(rest)?;
    if t == "string" {
        let (rest, _) = nom::character::complete::multispace0(rest)?;
        let (rest, num) = nom::character::complete::digit1(rest)?;
        let (rest, s) = nom::multi::count(
            nom::sequence::preceded(
                nom::character::complete::multispace0,
                nom::character::complete::alphanumeric1,
            ),
            num.parse().unwrap(),
        )(rest)?;
        let u_s: Vec<u8> = s.into_iter().map(|x| x.parse::<u8>().unwrap()).collect();
        let s = u_s.into_iter().map(|x| x as char).collect();
        let v = Val::String(s);
        let slot = vm.literal_slot(v);
        Ok((
            rest,
            Instruction {
                r_x: reg,
                r_y: 0,
                r_z: 0,
                opcode: opcode.clone(),
                slot,
                goto: 0,
            },
        ))
    } else if t == "true" {
        let slot = vm.literal_slot(Val::Bool(true));
        Ok((
            rest,
            Instruction {
                r_x: reg,
                r_y: 0,
                r_z: 0,
                opcode: opcode.clone(),
                slot,
                goto: 0,
            },
        ))
    } else if t == "false" {
        let slot = vm.literal_slot(Val::Bool(false));
        Ok((
            rest,
            Instruction {
                r_x: reg,
                r_y: 0,
                r_z: 0,
                opcode: opcode.clone(),
                slot,
                goto: 0,
            },
        ))
    } else if t == "nil" {
        let slot = vm.literal_slot(Val::Nil);
        Ok((
            rest,
            Instruction {
                r_x: reg,
                r_y: 0,
                r_z: 0,
                opcode: opcode.clone(),
                slot,
                goto: 0,
            },
        ))
    } else if t == "emptylist" {
        let slot = vm.literal_slot(Val::EmptyList);
        Ok((
            rest,
            Instruction {
                r_x: reg,
                r_y: 0,
                r_z: 0,
                opcode: opcode.clone(),
                slot,
                goto: 0,
            },
        ))
    } else {
        panic!("Unimplemented")
    }
}

fn parse_r1<'a>(opcode: &Opcodes, rest: &'a str) -> IResult<&'a str, Instruction> {
    let (rest, _) = nom::character::complete::multispace0(rest)?;
    let (rest, r_x) = nom::character::complete::digit1(rest)?;
    Ok((
        rest,
        Instruction {
            opcode: opcode.clone(),
            r_x: r_x.parse().unwrap(),
            r_y: 0,
            r_z: 0,
            slot: 0,
            goto: 0,
        },
    ))
}

fn parse_r3<'a>(opcode: &Opcodes, rest: &'a str) -> IResult<&'a str, Instruction> {
    let (rest, _) = nom::character::complete::multispace0(rest)?;
    let (rest, r_x) = nom::character::complete::digit1(rest)?;
    let (rest, _) = nom::character::complete::multispace0(rest)?;
    let (rest, r_y) = nom::character::complete::digit1(rest)?;
    let (rest, _) = nom::character::complete::multispace0(rest)?;
    let (rest, r_z) = nom::character::complete::digit1(rest)?;
    Ok((
        rest,
        Instruction {
            opcode: opcode.clone(),
            r_x: r_x.parse().unwrap(),
            r_y: r_y.parse().unwrap(),
            r_z: r_z.parse().unwrap(),
            slot: 0,
            goto: 0,
        },
    ))
}
