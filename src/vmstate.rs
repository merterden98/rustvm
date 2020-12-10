use std::collections::HashMap;

use crate::value::value::{VMFunction, Val};
use crate::vmstack::vmstack::Activation;
use colored::*;

#[derive(Debug)]
pub struct VMState {
    pub func: VMFunction,
    pub registers: Vec<Val>,
    pub globals: HashMap<Val, Val>,
    pub literals: Vec<Val>,
    pub stack: Vec<Activation>,
    pub test_suite: Tester,
}

#[derive(Debug)]
pub struct Tester {
    tests: u32,
    passed: u32,
    checkv: (Val, String),
}

impl Tester {
    pub fn check(&mut self, s: String, v: Val) -> () {
        self.checkv = (v, s)
    }
    pub fn expect(&mut self, _s: String, v: Val) -> () {
        self.tests += 1;
        if v != self.checkv.0 {
            println!("Got {:?}: Expected: {:?}", v, self.checkv.0);
            return;
        }
        self.passed += 1;
    }
    pub fn assert(&mut self, _s: String, v: Val) -> () {
        self.tests += 1;
        if v.as_bool() {
            self.passed += 1;
        }
    }
    pub fn report_tests(&self) -> () {
        if self.passed == self.tests {
            println!("{}", "All tests passed".green())
        } else {
            println!("{}", "Some tests failed".red())
        }
    }
}

pub fn init_vm_state() -> VMState {
    let func = VMFunction {
        arity: 0,
        nregs: 0,
        size: 0,
        instructions: Vec::new(),
    };
    let mut registers = Vec::with_capacity(50000);
    for _ in 0..registers.capacity() {
        registers.push(Val::Nil);
    }
    return VMState {
        func,
        registers,
        globals: HashMap::new(),
        literals: Vec::new(),
        stack: Vec::new(),
        test_suite: Tester {
            tests: 0,
            passed: 0,
            checkv: (Val::Nil, "".to_string()),
        },
    };
}

impl VMState {
    pub fn literal_slot(&mut self, v: Val) -> usize {
        self.literals.push(v);
        self.literals.len() - 1
    }
}
