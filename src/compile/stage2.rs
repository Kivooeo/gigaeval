use super::{
    stage0::{Error, Operand, Token},
    stage1::Stack,
};

pub struct UnwindStack<'a> {
    input_stack: Stack<'a>,
    output_queue: Stack<'a>,
    operator_stack: Stack<'a>,
}
impl<'a> UnwindStack<'a> {
    pub fn calculate(&mut self) -> Result<(), Error> {
        loop {
            if self.input_stack.is_empty() {
                break;
            }
            match self.input_stack.peek() {
                Some(Token::Operand(t)) if let Operand::Number(_) = t => {
                    self.output_queue.push(Token::Operand(t))
                }
                _ => break,
            }
        }

        Ok(())
    }
}
