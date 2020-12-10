pub mod vmstack {
    use crate::value::value::VMFunction;

    #[derive(Debug)]
    pub struct Activation {
        pub dest: usize,
        pub register_window: usize,
        pub program_counter: usize,
        pub fun: VMFunction,
    }
}
