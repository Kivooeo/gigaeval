use std::rc::Rc;

use super::stage0::Expression;
use super::stage0::Token;

type Node<'a> = Option<Rc<Stack<'a>>>;

#[derive(Debug)]
pub struct Stack<'a> {
    data: Node<'a>,
    value: Token<'a>,
    len: usize,
}

impl<'a> Stack<'a> {
    // Create a new stack with a single value
    pub fn new(value: Token<'a>) -> Self {
        Stack {
            data: None,
            value,
            len: 1,
        }
    }

    pub fn from(tokens: &'a mut Expression) -> Self {
        // Take the tokens and clear structurized
        let token_vec = tokens.structurized.take().expect("No tokens available");

        // Ensure there’s at least one token
        let mut iter = token_vec.into_iter();
        let first_token = iter.next().expect("No tokens available");

        // Initialize the stack with the first token
        let mut s = Stack::new(first_token);

        // Push remaining tokens in forward order
        for token in iter {
            s.push(token);
        }

        s
    }

    // Push a new value onto the stack
    pub fn push(&mut self, value: Token<'a>) {
        let new_stack = Stack {
            data: Some(Rc::new(std::mem::replace(
                self,
                Stack {
                    data: None,
                    value: value.clone(), // Use clone since Token implements Clone
                    len: 0,
                },
            ))),
            value,
            len: self.len + 1,
        };
        *self = new_stack;
    }

    // Pop the top element, returning the popped token
    pub fn pop(&mut self) -> Option<Token<'a>> {
        if self.len == 0 {
            return None;
        }

        let value_clone = self.value.clone(); // Clone for placeholder
        let popped_value = std::mem::replace(&mut self.value, value_clone.clone());
        if let Some(rc) = self.data.take() {
            let inner = Rc::try_unwrap(rc).unwrap_or_else(|_| panic!("Stack is still referenced"));
            *self = inner;
        } else {
            // Reset to empty state
            self.len = 0;
            self.data = None;
            self.value = value_clone; // Use cloned value as placeholder
        }
        Some(popped_value)
    }

    // Peek at the top value
    pub fn peek(&self) -> Option<Token<'a>> {
        Some(self.value.clone())
    }

    // Get the stack's length
    pub fn len(&self) -> usize {
        self.len
    }

    // Check if the stack is empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    // Pretty print of stack
    pub fn print(&self) {
        let mut values = Vec::with_capacity(self.len);
        let mut current = Some(self);

        // Collect references to values from top to bottom
        while let Some(node) = current {
            values.push(&node.value); // Store &Token<'a> instead of moving Token<'a>
            current = node.data.as_ref().map(|rc| rc.as_ref());
        }

        // Print from bottom to top
        println!("Stack (bottom to top, len={}):", self.len);
        for (i, value) in values.into_iter().enumerate() {
            println!("  {} :: {:?}", i + 1, value);
        }
    }

    pub fn next(&self) -> Option<Token<'a>> {
        if let Some(next_node) = self.data.clone() {
            return Some(next_node.value.clone());
        } else {
            None
        }
    }
}
