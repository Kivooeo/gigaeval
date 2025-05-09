/// Stage 0 means tokenizin
/// And assignes variables
#[derive(Debug)]
pub enum Function {
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Sinh,
    Cosh,
    Tanh,
    Asinh,
    Acosh,
    Atanh,
    Exp,
    Log,
    Log2,
    Log10,
    Sqrt,
    Abs,
    Ceil,
    Floor,
    Round,
    Trunc,
}
#[derive(Debug)]

pub enum Operand<'a> {
    Number(f64),
    Variable(&'a str),
    Function(Function),
}
#[derive(Debug)]

pub enum Operator {
    Plus,
    Uplus,
    Minus,
    Uminus,
    Multiply,
    Divide,
    Power,
    Modulus,
}
#[derive(Debug)]

pub enum Token<'a> {
    Operand(Operand<'a>),
    Operator(Operator),
    ParenL,
    ParenR,
}

impl<'a> Token<'a> {
    fn is_integer(&self) -> bool {
        match &self {
            Token::Operand(_) => true,
            _ => false,
        }
    }
    fn is_operator(&self) -> bool {
        match &self {
            Token::Operator(_) => true,
            _ => false,
        }
    }
    fn is_paren_l(&self) -> bool {
        match &self {
            Token::ParenL => true,
            _ => false,
        }
    }
    fn is_paren_r(&self) -> bool {
        match &self {
            Token::ParenR => true,
            _ => false,
        }
    }
}

/// The expression struct is used to store the raw expression and the tokenized version of it
/// The tokenized version is used to parse the expression and evaluate it
#[derive(Debug)]

pub struct Expression<'a> {
    raw: &'a str,
    structurized: Option<Vec<Token<'a>>>,
}
#[derive(Debug)]
pub enum Error {
    InvalidExpression(usize),
    InvalidToken((usize, char)),
    InvalidFunction(usize),
    InvalidOperator(usize),
    InvalidOperand((usize, String)),
    InvalidVariable(usize),
    InvalidParenthesis(usize),
    InvalidEOF(usize),
}

impl<'a> Expression<'a> {
    pub fn new(raw: &'a str) -> Self {
        Expression {
            raw,
            structurized: None,
        }
    }

    // Function to stringify tokens sequence
    pub fn lookup(&self) -> String {
        let mut string: String = String::new();

        for token in self.structurized.as_ref().unwrap() {
            let token_str = match token {
                Token::Operand(operand) => match operand {
                    Operand::Number(num) => num.to_string(),
                    Operand::Variable(var) => var.to_string(),
                    _ => "func".to_owned(),
                },

                Token::Operator(op) => match op {
                    Operator::Plus => "+".to_string(),
                    Operator::Minus => "-".to_string(),
                    Operator::Multiply => "*".to_string(),
                    Operator::Divide => "/".to_string(),
                    Operator::Power => "^".to_string(),
                    Operator::Modulus => "%".to_string(),
                    Operator::Uplus => "+".to_string(),
                    Operator::Uminus => "-".to_string(),
                },

                Token::ParenL => "(".to_string(),
                Token::ParenR => ")".to_string(),
            };

            if !string.is_empty() && !matches!(token, Token::ParenR) {
                string.push_str(" ");
            }
            string.push_str(&token_str);
        }

        string
    }

    pub fn tokenize(&mut self) -> Result<(), Error> {
        if self.structurized.is_some() {
            return Err(Error::InvalidEOF(0));
        }

        let mut tokens: Vec<Token<'a>> = Vec::new();
        let mut current_pos: usize = 0;
        let mut paren_balance: isize = 0;

        // We'll use this macro for incrementing current_pos
        // Just to make less boilerplate ^^
        macro_rules! inc {
            () => {
                current_pos += 1
            };
        }

        loop {
            if current_pos >= self.raw.len() {
                if paren_balance != 0 {
                    return Err(Error::InvalidParenthesis(0));
                }
                self.structurized = Some(tokens);
                break;
            }
            // dbg!(&tokens);
            let mut unary_check = tokens
                .last()
                .map_or(false, |x| x.is_integer() || x.is_paren_r());
            match self.raw.chars().nth(current_pos) {
                Some(token) => match token {
                    token if token.is_numeric() || token == '.' => {
                        let mut final_number = String::new();
                        let mut dot_seen = token == '.';
                        if dot_seen {
                            final_number.push('.');
                            inc!();
                        }

                        while let Some(next_char) = self.raw.chars().nth(current_pos) {
                            if next_char.is_numeric() {
                                final_number.push(next_char);
                                inc!();
                            } else if next_char == '.' && !dot_seen {
                                final_number.push('.');
                                dot_seen = true;
                                inc!();
                            } else {
                                break;
                            }
                        }

                        match final_number.parse::<f64>() {
                            Ok(number) => tokens.push(Token::Operand(Operand::Number(number))),
                            Err(_) => {
                                return Err(Error::InvalidOperand((current_pos, final_number)));
                            }
                        }
                    }
                    '+' | '-' => {
                        match token {
                            '+' => tokens.push(Token::Operator(if unary_check {
                                Operator::Plus
                            } else {
                                Operator::Uplus
                            })),
                            '-' => tokens.push(Token::Operator(if unary_check {
                                Operator::Minus
                            } else {
                                Operator::Uminus
                            })),
                            // safety: we will never reach this panic
                            // because we just checked operator and it can be only
                            // plus or minus
                            _ => panic!("uwu"),
                        }
                        inc!()
                    }
                    '*' | '/' | '^' | '%' => {
                        // check if operator unary or not
                        // this should be NOT unary to work
                        if !unary_check {
                            return Err(Error::InvalidOperator(current_pos));
                        }
                        match token {
                            '*' => tokens.push(Token::Operator(Operator::Multiply)),
                            '/' => tokens.push(Token::Operator(Operator::Divide)),
                            '^' => tokens.push(Token::Operator(Operator::Power)),
                            '%' => tokens.push(Token::Operator(Operator::Modulus)),
                            // safety: we will never reach this panic
                            // because we just checked operator and it can be only
                            // mul, div, mod or pow
                            _ => panic!("uwu"),
                        }
                        inc!()
                    }
                    '(' => {
                        paren_balance += 1;
                        tokens.push(Token::ParenL);
                        inc!()
                    }
                    ')' => {
                        paren_balance -= 1;
                        if paren_balance < 0 {
                            return Err(Error::InvalidParenthesis(current_pos));
                        }
                        tokens.push(Token::ParenR);
                        inc!()
                    }
                    whitespace if whitespace.is_whitespace() => inc!(),
                    // At this point, we get something really bad
                    // So we raising InvalidToken and say goodbye to user
                    // It was pleasure to work with you ^^
                    bad_token => return Err(Error::InvalidToken((current_pos, bad_token))),
                },

                None => return Err(Error::InvalidEOF(0)),
            }
        }

        Ok(())
    }
}
