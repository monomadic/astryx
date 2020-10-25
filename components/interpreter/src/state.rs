use crate::{models::Object, InterpreterError, InterpreterResult};
use parser::{Expression, FunctionCall, StringToken};
use std::{cell::RefCell, collections::HashMap, fs::OpenOptions, io::Write, rc::Rc};

type LocalData<'a> = HashMap<String, Object<'a>>;

#[derive(Debug, Clone)]
pub enum Writer {
    None,
    StdOut,
    File(String),
    Buffer(Vec<u8>),
}

#[derive(Debug, Clone)]
pub struct State<'a> {
    local: LocalData<'a>,
    outer: Option<Rc<RefCell<State>>>,
    pub writer: Writer,
}

impl<'a> State<'a> {
    pub fn new() -> Self {
        State {
            local: LocalData::new(),
            // document: Node::new(AstryxNode::Root),
            writer: Writer::None,
        }
    }

    /// bind a variable to local state
    pub fn bind(&mut self, ident: &str, obj: Object<'a>) -> InterpreterResult<()> {
        let _ = self.locals.insert(ident.into(), obj); // return doesn't matter as all state is mutable
        Ok(()) // force return ok (this could change if mutability rules change)
    }

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

    // pub fn eval_statement()

    pub fn eval_expression(&self, expr: &Expression) -> InterpreterResult<Object> {
        Ok(match expr {
            Expression::FunctionCall(f) => self.eval_function(&f)?,
            Expression::Reference(r) => Object::String(format!("r{:?}", r)),
            Expression::Literal(l) => Object::String(l.to_string()),
        })
    }

    /// execute a function
    pub fn eval_function(&self, f: &FunctionCall) -> InterpreterResult<Object> {
        // get the function expression from the state
        let func = self.eval_expression(&f.ident)?;

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
    pub fn interpolate(&self, components: Vec<StringToken>) -> InterpreterResult<String> {
        Ok(components
            .into_iter()
            .map(|st| match st {
                StringToken::Text(span) => Ok(span.fragment().to_string()),
                // StringToken::Expression(expr) => self.eval(&expr).map(|e| e.into()),
                StringToken::Expression(expr) => Ok(format!("expression")),
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
