
use crate::{models::AstryxNode, InterpreterResult, state::State};
use parser::Statement;

pub(crate) fn interpret(_ast: &Statement, _state: State) -> InterpreterResult<AstryxNode> {
    Ok(AstryxNode::Element)
}
