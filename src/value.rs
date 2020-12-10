pub mod value {
    use std::fmt::{self, Display};

    use crate::opcodes::Instruction;

    #[derive(Debug, Clone, Hash)]
    pub enum Val {
        Nil,
        EmptyList,
        Cons(Box<Val>, Box<Val>),
        Num(i32),
        Bool(bool),
        String(String),
        VMFunction(VMFunction),
        Closure(VMFunction, Vec<Val>),
    }

    impl Val {
        pub fn as_num(&self) -> i32 {
            match self {
                Val::Num(i) => *i,
                _ => panic!("Can't be interpreted as a number"),
            }
        }
        pub fn as_bool(&self) -> bool {
            match self {
                Val::Nil => false,
                Val::EmptyList => false,
                Val::Num(i) => (i > &0),
                Val::Bool(b) => b.clone(),
                Val::String(_) => true,
                Val::VMFunction(_) => true,
                Val::Cons(_, _) => true,
                Val::Closure(_, _) => true,
            }
        }
        pub fn as_string(&self) -> String {
            match self {
                Val::String(s) => s.clone(),
                _ => panic!("Can't make a string from non-string VM Value"),
            }
        }
        pub fn to_num(n: i32) -> Self {
            Val::Num(n)
        }
    }
    impl Display for Val {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Val::Nil => write!(f, "{}", "nil"),
                Val::Num(i) => write!(f, "{}", i),
                Val::Bool(b) => write!(f, "{}", b),
                Val::String(s) => write!(f, "{}", s),
                Val::VMFunction(_) => write!(f, "A VM Function"),
                Val::EmptyList => write!(f, "'()"),
                Val::Cons(x, xs) => write!(f, "{} {}", x, xs),
                Val::Closure(_f, _cl) => write!(f, "A Vm Closure {:?}", _cl),
            }
        }
    }

    impl PartialEq for Val {
        fn eq(&self, other: &Self) -> bool {
            match self {
                Val::Nil => match other {
                    Val::Nil => true,
                    _ => false,
                },
                Val::EmptyList => match other {
                    Val::EmptyList => true,
                    _ => false,
                },
                Val::Num(i) => match other {
                    Val::Num(j) => i == j,
                    _ => false,
                },
                Val::Bool(b) => match other {
                    Val::Bool(b2) => b2 == b,
                    _ => false,
                },
                Val::String(s1) => match other {
                    Val::String(s2) => s1 == s2,
                    _ => false,
                },
                Val::VMFunction(_) => false,
                Val::Cons(x, xs) => match other {
                    Val::Cons(y, ys) => y == x && xs == ys,
                    _ => false,
                },
                Val::Closure(_, _) => false,
            }
        }
    }
    impl Eq for Val {}
    #[derive(Debug, Clone, Hash)]
    pub struct VMFunction {
        pub arity: i32,
        pub nregs: i32,
        pub size: i32,
        pub instructions: Vec<Instruction>,
    }
}
