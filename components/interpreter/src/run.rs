
use crate::{models::AstryxNode, InterpreterResult, state::State};
use parser::Statement;

pub(crate) fn interpret(ast: Vec<Statement>, state: State) -> InterpreterResult<AstryxNode> {
    Ok(AstryxNode::Element)
}
