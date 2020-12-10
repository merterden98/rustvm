use crate::{
    value::{self, value::VMFunction},
    vmstack::vmstack::Activation,
    vmstate::VMState,
};
use value::value::Val;

pub fn run(vm: &mut VMState, function: &mut VMFunction) -> () {
    let mut i = 0;
    let mut reg_window = 0;
    loop {
        if i >= function.instructions.len() {
            return;
        }
        let instruction = function.instructions[i];
        i += 1;
        let x = vm
            .registers
            .get(reg_window + instruction.r_x)
            .expect("Expected something in register x")
            .clone();
        let y = vm
            .registers
            .get(reg_window + instruction.r_y)
            .expect("Expected something in register y")
            .clone();
        let z = vm
            .registers
            .get(reg_window + instruction.r_z)
            .expect("Expected something in register z")
            .clone();

        match instruction.opcode {
            crate::opcodes::Opcodes::Add => {
                let num = Val::to_num(Val::as_num(&y) + Val::as_num(&z));
                vm.registers[reg_window + instruction.r_x] = num;
            }
            crate::opcodes::Opcodes::LoadLiteral => {
                if let Some(v) = vm.literals.get(instruction.slot) {
                    vm.registers[reg_window + instruction.r_x] = v.clone();
                }
            }
            crate::opcodes::Opcodes::Print => {
                if let Some(x) = vm.registers.get(reg_window + instruction.r_x) {
                    println!("{}", x);
                }
            }
            crate::opcodes::Opcodes::Halt => return,
            crate::opcodes::Opcodes::Goto => match instruction.goto.is_positive() {
                true => i += instruction.goto as usize - 1,
                false => i -= instruction.goto.abs() as usize + 1,
            },
            crate::opcodes::Opcodes::Not => {
                vm.registers[reg_window + instruction.r_y] = Val::Bool(!Val::as_bool(&y))
            }
            crate::opcodes::Opcodes::Mov => {
                vm.registers[reg_window + instruction.r_x] = y.clone();
            }
            crate::opcodes::Opcodes::If => {
                if !Val::as_bool(&vm.registers[reg_window + instruction.r_x]) {
                    i += 1;
                }
            }
            crate::opcodes::Opcodes::Subtract => {
                let num = Val::to_num(Val::as_num(&y) - Val::as_num(&z));
                vm.registers[reg_window + instruction.r_x] = num;
            }
            crate::opcodes::Opcodes::Multiply => {
                let num = Val::to_num(Val::as_num(&y) * Val::as_num(&z));
                vm.registers[reg_window + instruction.r_x] = num;
            }
            crate::opcodes::Opcodes::Divide => {
                let num = Val::to_num(Val::as_num(&y) / Val::as_num(&z));
                vm.registers[reg_window + instruction.r_x] = num;
            }
            crate::opcodes::Opcodes::Equal => {
                let num = Val::Bool(y == z);
                vm.registers[reg_window + instruction.r_x] = num;
            }
            crate::opcodes::Opcodes::Check => {
                if let Some(v) = vm.literals.get(instruction.slot) {
                    vm.test_suite.check(
                        v.as_string(),
                        vm.registers[reg_window + instruction.r_x].clone(),
                    )
                }
            }
            crate::opcodes::Opcodes::Expect => {
                if let Some(v) = vm.literals.get(instruction.slot) {
                    vm.test_suite.expect(
                        v.as_string(),
                        vm.registers[reg_window + instruction.r_x].clone(),
                    )
                }
            }
            crate::opcodes::Opcodes::SetGlobal => {
                vm.globals.insert(
                    vm.literals[instruction.slot].clone(),
                    vm.registers[reg_window + instruction.r_x].clone(),
                );
                ()
            }
            crate::opcodes::Opcodes::GetGlobal => {
                vm.registers[reg_window + instruction.r_x] = vm
                    .globals
                    .get(&vm.literals[instruction.slot])
                    .expect(&format!("Expected {}", vm.literals[instruction.slot]))
                    .clone();
            }
            crate::opcodes::Opcodes::IsSymbol => {
                vm.registers[reg_window + instruction.r_x] = match y {
                    Val::String(_) => Val::Bool(true),
                    _ => Val::Bool(false),
                }
            }
            crate::opcodes::Opcodes::IsBoolean => {
                vm.registers[reg_window + instruction.r_x] = match y {
                    Val::Bool(_) => Val::Bool(true),
                    _ => Val::Bool(false),
                }
            }
            crate::opcodes::Opcodes::IsNil => {
                vm.registers[reg_window + instruction.r_x] = match y {
                    Val::Nil => Val::Bool(true),
                    _ => Val::Bool(false),
                }
            }
            crate::opcodes::Opcodes::IsNull => {
                vm.registers[reg_window + instruction.r_x] = match y {
                    Val::EmptyList => Val::Bool(true),
                    _ => Val::Bool(false),
                }
            }
            crate::opcodes::Opcodes::IsNumber => {
                vm.registers[reg_window + instruction.r_x] = match y {
                    Val::Num(_) => Val::Bool(true),
                    _ => Val::Bool(false),
                }
            }
            crate::opcodes::Opcodes::Greater => {
                let num = Val::Bool(y.as_num() > z.as_num());
                vm.registers[reg_window + instruction.r_x] = num;
            }
            crate::opcodes::Opcodes::Less => {
                let num = Val::Bool(y.as_num() < z.as_num());
                vm.registers[reg_window + instruction.r_x] = num;
            }
            crate::opcodes::Opcodes::LessEq => {
                let num = Val::Bool(y.as_num() <= z.as_num());
                vm.registers[reg_window + instruction.r_x] = num;
            }
            crate::opcodes::Opcodes::Return => {
                let act = vm.stack.pop().unwrap();
                *function = act.fun;
                i = act.program_counter;
                vm.registers[act.dest] = vm.registers[reg_window + instruction.r_x].clone();
                reg_window = act.register_window;
            }
            crate::opcodes::Opcodes::Call => match y {
                Val::VMFunction(f) => {
                    let act = Activation {
                        dest: reg_window + instruction.r_x,
                        register_window: reg_window,
                        program_counter: i,
                        fun: function.clone(),
                    };
                    reg_window += instruction.r_z - instruction.r_y - 1;
                    vm.stack.push(act);
                    reg_window += instruction.r_y;
                    *function = f.clone();
                    i = 0;
                }
                Val::Closure(f, _) => {
                    let act = Activation {
                        dest: reg_window + instruction.r_x,
                        register_window: reg_window,
                        program_counter: i,
                        fun: function.clone(),
                    };
                    reg_window += instruction.r_z - instruction.r_y - 1;
                    vm.stack.push(act);
                    reg_window += instruction.r_y;
                    *function = f.clone();
                    i = 0;
                }
                _ => {
                    panic!("Can't Call something that isn't a function")
                }
            },
            crate::opcodes::Opcodes::TailCall => match x {
                Val::VMFunction(f) => {
                    for i in 0..(instruction.r_y - instruction.r_x + 1) {
                        vm.registers
                            .swap(reg_window + i, reg_window + i + instruction.r_x);
                    }

                    *function = f.clone();
                    i = 0;
                }
                Val::Closure(f, _) => {
                    for i in 0..(instruction.r_y - instruction.r_x + 1) {
                        vm.registers
                            .swap(reg_window + i, reg_window + i + instruction.r_x);
                    }

                    *function = f.clone();
                    i = 0;
                }
                _ => {
                    panic!("Can't Call something that isn't a function got {:?}", x)
                }
            },
            crate::opcodes::Opcodes::Cons => {
                vm.registers[reg_window + instruction.r_x] =
                    Val::Cons(Box::new(y.clone()), Box::new(z.clone()));
            }
            crate::opcodes::Opcodes::Car => {
                vm.registers[reg_window + instruction.r_x] = match y {
                    Val::Cons(x, _) => *x.clone(),
                    _ => panic!("attempted to car: {}", x),
                }
            }
            crate::opcodes::Opcodes::Cdr => {
                vm.registers[reg_window + instruction.r_x] = match y {
                    Val::Cons(_, xs) => *xs.clone(),
                    _ => panic!("attempted to cdr: {}", y),
                }
            }
            crate::opcodes::Opcodes::MakeClosure => match y {
                Val::VMFunction(f) => {
                    vm.registers[reg_window + instruction.r_x] =
                        Val::Closure(f.clone(), vec![Val::Nil; instruction.r_z]);
                }
                _ => panic!("Attempted to make a closure without a function"),
            },
            crate::opcodes::Opcodes::SetClSlot => match x {
                Val::Closure(f, v) => {
                    let mut new_v = v.clone();
                    let new_f = f.clone();
                    new_v[instruction.r_z] = y.clone();
                    vm.registers[instruction.r_x + reg_window] = Val::Closure(new_f, new_v);
                }
                _ => panic!("Attempted to set a non closure"),
            },
            crate::opcodes::Opcodes::GetClSlot => match y {
                Val::Closure(_, v) => {
                    vm.registers[reg_window + instruction.r_x] = v[instruction.r_z].clone()
                }
                _ => panic!("Attempted to set a non closure"),
            },
            crate::opcodes::Opcodes::SetCar => {
                let x = vm.registers.get_mut(instruction.r_x).unwrap();
                match x {
                    Val::Cons(hd, _) => **hd = y.clone(),
                    _ => panic!("error"),
                }
                ()
            }
            crate::opcodes::Opcodes::SetCdr => {
                let x = vm.registers.get_mut(instruction.r_x).unwrap();
                match x {
                    Val::Cons(_, tail) => **tail = y.clone(),
                    _ => panic!("error"),
                }
                ()
            }
            crate::opcodes::Opcodes::NotEqual => {}
            crate::opcodes::Opcodes::Assert => {
                vm.test_suite
                    .assert(vm.literals[instruction.slot].as_string(), x);
            }
            crate::opcodes::Opcodes::IDiv => {
                let num = Val::to_num(Val::as_num(&y) / Val::as_num(&z));
                vm.registers[reg_window + instruction.r_x] = num;
            }
            crate::opcodes::Opcodes::Pair => {
                todo!()
            }
            crate::opcodes::Opcodes::Error => {
                todo!()
            }
        }
    }
}
