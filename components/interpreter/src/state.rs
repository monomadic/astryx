use crate::{models::Object, InterpreterError, InterpreterResult};
use parser::{Expression, FunctionCall, Span, StringToken};
use std::{cell::RefCell, collections::HashMap, fs::OpenOptions, io::Write, rc::Rc};

type LocalData<'a> = HashMap<String, Object<'a>>;

#[derive(Debug, Clone)]
pub enum Writer {
    None,
    StdOut,
    File(String),
    Buffer(Vec<u8>),
}

impl Default for Writer {
    fn default() -> Self {
        Writer::None
    }
}

#[derive(Clone, Default)]
pub struct State<'a> {
    local: LocalData<'a>,
    outer: Option<Rc<RefCell<State<'a>>>>,
    pub writer: Writer,
}

impl<'a> State<'a> {
    // replace with default()
    pub fn new() -> Self {
        State {
            local: LocalData::new(),
            // document: Node::new(AstryxNode::Root),
            writer: Writer::None,
            outer: None,
        }
    }

    pub fn get(&self, name: &str) -> Option<Object<'a>> {
        match self.local.get(name) {
            Some(value) => Some(value.clone()),
            None => self
                .outer
                .as_ref()
                .and_then(|o| o.borrow().get(name).clone()),
        }
    }

    /// fetch a variable from state and throw an error upon failure
    pub fn require(&self, name: &str) -> InterpreterResult<Object<'a>> {
        self.get(name)
            .ok_or(InterpreterError::InvalidReference(String::from(name)))
    }

    /// bind a variable to local state
    pub fn bind(&mut self, ident: &str, obj: Object<'a>) -> InterpreterResult<()> {
        let _ = self.local.insert(ident.into(), obj); // return doesn't matter as all state is mutable
        Ok(()) // force return ok (this could change if mutability rules change, or overwriting builtins)
    }

    pub fn extend(outer: Rc<RefCell<Self>>) -> Self {
        Self {
            outer: Some(outer),
            ..Default::default()
        }
    }

    // pub fn root(&self) -> Rc<RefCell<State<'a>>> {
    //     match self.outer {
    //         Some(o) => o.borrow().root(),
    //         None => Rc::new(RefCell::new(self)),
    //     }
    // }

    pub fn get_mut_writer(&mut self) -> InterpreterResult<Box<dyn Write>> {
        match &self.writer {
            Writer::None => Err(InterpreterError::NoWriter),
            Writer::StdOut => Ok(Box::new(std::io::stdout())),
            Writer::File(path) => Ok(Box::new(
                OpenOptions::new().append(true).open(path).expect("45"),
            )),
            Writer::Buffer(b) => Ok(Box::new(b.to_vec())),
        }
    }

    pub fn eval_function_arguments(
        &self,
        args: &Vec<(Span<'a>, Expression<'a>)>,
    ) -> InterpreterResult<Vec<Object<'a>>> {
        args.into_iter()
            .map(|(_ident, expr)| self.eval_expression(expr))
            .collect::<Result<Vec<Object>, InterpreterError>>()
    }

    pub fn eval_expression(&self, expr: &Expression<'a>) -> InterpreterResult<Object<'a>> {
        match expr {
            Expression::FunctionCall(f) => self.eval_function(&f),
            Expression::Reference(r) => self.require(r.to_string().as_str()),
            Expression::Literal(l) => Ok(Object::String(l.to_string())),
            Expression::RelativePath(_) => unimplemented!(),
            Expression::Array(_) => unimplemented!(),
            Expression::GlobPattern(s) => crate::util::import_files(s),
            Expression::Index(l, r) => {
                let lexpr = self.eval_expression(l)?;

                match lexpr {
                    Object::Map(ref m) => match **r {
                        Expression::Reference(r) => m
                            .get(r.to_string().as_str())
                            .map(|o| o.clone())
                            .ok_or(InterpreterError::UnknownMemberFunction(r.to_string())),
                        _ => Err(InterpreterError::Generic("aa".into())),
                    },
                    _ => Err(InterpreterError::Generic("bb".into())),
                }
            }
        }
    }

    /// execute a function
    pub fn eval_function(&self, f: &FunctionCall<'a>) -> InterpreterResult<Object<'a>> {
        // get the function expression from the state
        let func = self.eval_expression(&f.ident)?;

        // let state = Rc::clone(self);

        // // find function in local state
        // match state.locals.get(&ident) {
        //     Some(r) => {}
        //     None => {
        //         return Err(InterpreterError::FunctionNotFound(ident));
        //     }
        // }

        // eval(, state);
        Ok(Object::String(format!("f--{:?}", f)))
    }

    /// Convert string tokens to a fully interpolated string
    pub fn interpolate(&self, components: Vec<StringToken<'a>>) -> InterpreterResult<String> {
        Ok(components
            .into_iter()
            .map(|st| match st {
                StringToken::Text(span) => Ok(span.to_string()),
                // StringToken::Expression(expr) => self.eval(&expr).map(|e| e.into()),
                StringToken::Expression(expr) => Ok(self.eval_expression(&expr)?.to_string()),
            })
            .collect::<Result<Vec<String>, InterpreterError>>()?
            .into_iter()
            .collect())
    }

    pub fn write(&mut self, i: &str) -> InterpreterResult<()> {
        let mut writer = self.get_mut_writer()?;

        writer
            .write_fmt(format_args!("{}", i))
            .map_err(|_| InterpreterError::Generic("IO".into()))
    }
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
