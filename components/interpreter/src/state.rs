use crate::{
    eval::eval_expression, models::Object, InterpreterError, InterpreterErrorKind,
    InterpreterResult,
};
use parser::{Expression, FunctionCall, Span, StringToken};
use program::ProgramInstruction;
use std::{cell::RefCell, collections::HashMap, fs::OpenOptions, io::Write, rc::Rc};

type LocalData = HashMap<String, Object>;

// #[derive(Debug, Clone)]
// pub enum Writer {
//     None,
//     StdOut,
//     File(String),
//     Buffer(Vec<u8>),
// }

// impl Default for Writer {
//     fn default() -> Self {
//         Writer::None
//     }
// }

#[derive(Clone, Default)]
pub struct State {
    pub local: LocalData,
    outer: Option<Rc<RefCell<State>>>,
    program: Rc<RefCell<Vec<ProgramInstruction>>>,
    // pub writer: Writer,
}

impl<'a> State {
    // replace with default()
    pub fn new() -> Self {
        State {
            local: LocalData::new(),
            // writer: Writer::None,
            outer: None,
            program: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        match self.local.get(name) {
            Some(value) => Some(value.clone()),
            None => self
                .outer
                .as_ref()
                .and_then(|o| o.borrow().get(name).clone()),
        }
    }

    /// fetch a variable from state and throw an error upon failure
    pub fn require(&self, name: &Span) -> InterpreterResult<Object> {
        self.get(&name.to_string()).ok_or(InterpreterError {
            kind: InterpreterErrorKind::InvalidReference(name.to_string()),
            location: Some((*name).into()),
        })
    }

    /// bind a variable to local state
    pub fn bind(&mut self, ident: &str, obj: Object) -> InterpreterResult<()> {
        let _ = self.local.insert(ident.into(), obj.clone()); // return doesn't matter as all state is mutable
        Ok(()) // force return ok (this could change if mutability rules change, or overwriting builtins)
    }

    pub fn push_instruction(&self, instruction: ProgramInstruction) {
        self.program.borrow_mut().push(instruction)
    }

    pub fn get_program(&self) -> Vec<ProgramInstruction> {
        self.program.borrow().clone()
    }

    pub fn extend(outer: Rc<RefCell<Self>>) -> Self {
        Self {
            outer: Some(outer),
            ..Default::default()
        }
    }

    /// returns a flattened hashmap of all objects in state
    pub fn to_map(&self) -> HashMap<String, Object> {
        self.local.clone() // todo: inherit
    }

    // pub fn root(&self) -> Rc<RefCell<State<'a>>> {
    //     match self.outer {
    //         Some(o) => o.borrow().root(),
    //         None => Rc::new(RefCell::new(self)),
    //     }
    // }

    // pub fn get_mut_writer(&mut self) -> InterpreterResult<Box<dyn Write>> {
    //     match &self.writer {
    //         Writer::None => unreachable!(), // remove this
    //         Writer::StdOut => Ok(Box::new(std::io::stdout())),
    //         Writer::File(path) => Ok(Box::new(
    //             OpenOptions::new().append(true).open(path).expect("45"),
    //         )),
    //         Writer::Buffer(b) => Ok(Box::new(b.to_vec())),
    //     }
    // }

    pub fn eval_function_arguments(
        &self,
        args: &Vec<(Span<'a>, Expression<'a>)>,
    ) -> InterpreterResult<Vec<Object>> {
        unimplemented!();
        // args.into_iter()
        //     .map(|(_ident, expr)| self.eval_expression(expr))
        //     .collect::<Result<Vec<Object>, InterpreterError>>()
    }

    // pub fn eval_expression(&self, expr: &Expression<'a>) -> InterpreterResult<Object> {
    //     unimplemented!();
    //     match expr {
    //         Expression::FunctionCall(f) => self.eval_function(&f),
    //         Expression::Reference(r) => self.require(r),
    //         Expression::Literal(l) => match l {
    //             parser::Literal::String(s) => Ok(Object::String(s.to_string())),
    //             parser::Literal::Number(s, f) => unimplemented!(),
    //         },
    //         Expression::RelativePath(_) => unimplemented!(),
    //         Expression::Array(_) => unimplemented!(),
    //         Expression::GlobPattern(s) => crate::util::import_files(s),
    //         Expression::Index(l, r) => {
    //             let lexpr = self.eval_expression(l)?;

    //             match lexpr {
    //                 Object::Map(ref m) => match **r {
    //                     Expression::Reference(r) => m
    //                         .get(r.to_string().as_str())
    //                         .map(|o| o.clone())
    //                         .ok_or(InterpreterError {
    //                             kind: InterpreterErrorKind::UnknownMemberFunction(r.to_string()),
    //                             location: Some(r.into()),
    //                         }),
    //                     _ => unimplemented!(),
    //                 },
    //                 Object::String(s) => match &**r {
    //                     Expression::FunctionCall(f) => {
    //                         // get the function closure from local state as Object::BuiltInFunction(f)
    //                         let func: Object =
    //                             self.eval_expression(&f.ident)
    //                                 .map_err(|e| InterpreterError {
    //                                     kind: InterpreterErrorKind::FunctionNotFound(
    //                                         f.ident.inspect(),
    //                                     ),
    //                                     location: e.location,
    //                                 })?;

    //                         // DOUBLE, REMOVE: this needs to be rewritten, to put the arguments into a
    //                         // new scope and send that to the function closure.
    //                         let mut args = f
    //                             .arguments
    //                             .iter()
    //                             .map(|(_ident, expr)| self.eval_expression(expr))
    //                             .collect::<Result<Vec<Object>, _>>()?;

    //                         args.push(Object::String(s));

    //                         // let inner = Rc::new(RefCell::new(*self));

    //                         unimplemented!()

    //                         // let inner = State::extend(Rc::new(RefCell::new(*self)));

    //                         // let obj = match func {
    //                         //     Object::BuiltinFunction(builtin) => {
    //                         //         builtin(Rc::new(RefCell::new(inner)))?
    //                         //     }
    //                         //     _ => unimplemented!(),
    //                         // };

    //                         // Ok(obj)
    //                     }
    //                     _ => unimplemented!(),
    //                 },
    //                 _ => panic!("{}", lexpr.inspect()),
    //             }
    //         }
    //     }
    // }

    // /// execute a function
    // pub fn eval_function(&self, f: &FunctionCall<'a>) -> InterpreterResult<Object> {
    //     // get the function closure from local state as Object::BuiltInFunction(f)
    //     let func: Object =
    //         eval_expression(Rc::clone(&self), &f.ident).map_err(|e| InterpreterError {
    //             kind: InterpreterErrorKind::FunctionNotFound(f.ident.inspect()),
    //             location: e.location,
    //         })?;

    //     // this needs to be rewritten, to put the arguments into a
    //     // new scope and send that to the function closure.
    //     let args = f
    //         .arguments
    //         .iter()
    //         .map(|(_ident, expr)| self.eval_expression(expr))
    //         .collect::<Result<Vec<Object>, _>>()?;

    //     unimplemented!()
    //     // fix this... jeez
    //     // let inner = Rc::new(RefCell::new(State::extend(Rc::new(RefCell::new(*self)))));

    //     // let obj = match func {
    //     //     Object::BuiltinFunction(builtin) => builtin(inner)?,
    //     //     _ => unimplemented!(),
    //     // };

    //     // eval(, state);
    //     // Ok(obj)
    // }

    // /// Convert string tokens to a fully interpolated string
    // pub fn interpolate(&self, components: Vec<StringToken<'a>>) -> InterpreterResult<String> {
    //     Ok(components
    //         .into_iter()
    //         .map(|st| match st {
    //             StringToken::Text(span) => Ok(span.to_string()),
    //             // StringToken::Expression(expr) => self.eval(&expr).map(|e| e.into()),
    //             StringToken::Expression(expr) => {
    //                 Ok(eval_expression(Rc::new(RefCell::new(*self)), &expr)?.to_string())
    //             }
    //         })
    //         .collect::<Result<Vec<String>, InterpreterError>>()?
    //         .into_iter()
    //         .collect())
    // }

    // pub fn write(&mut self, i: &str) -> InterpreterResult<()> {
    //     let mut writer = self.get_mut_writer()?;

    //     writer
    //         .write_fmt(format_args!("{}", i))
    //         .map_err(|_| InterpreterError {
    //             kind: InterpreterErrorKind::IOError,
    //             location: None,
    //         })
    // }
}

// pub fn eval_expression<'a>(
//     state: Rc<RefCell<State>>,
//     expr: &Expression,
// ) -> InterpreterResult<Object<'a>> {
//     Ok(match expr {
//         // Expression::FunctionCall(f) => state.eval_function(&f)?,
//         Expression::FunctionCall(f) => Object::String(format!("f{:?}", f)),
//         Expression::Reference(r) => Object::String(format!("r{:?}", r)),
//         Expression::Literal(l) => Object::String(l.to_string()),
//         Expression::RelativePath(_) => unimplemented!(),
//         Expression::Array(_) => unimplemented!(),
//     })
// }
