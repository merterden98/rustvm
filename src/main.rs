mod loader;
mod opcodes;
mod value;
mod vmrun;
mod vmstack;
mod vmstate;
use either::Either;
use std::env;
use std::fs;
use std::io;
use vmrun::run;

use vmstate::init_vm_state;
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut state = init_vm_state();
    let parser_map = opcodes::get_parsers();
    if args.len() == 1 {
        let mut vm_function =
            loader::load_modules(Either::Left(io::stdin()), &parser_map, &mut state);
        run(&mut state, &mut vm_function);
    } else {
        let my_file = fs::File::open(args.get(1).unwrap()).expect("Failed to open file");

        let mut vm_function = loader::load_modules(Either::Right(my_file), &parser_map, &mut state);
        run(&mut state, &mut vm_function);
    }
    state.test_suite.report_tests();
    ()
}
