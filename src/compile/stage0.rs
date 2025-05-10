use std::collections::HashMap;

/// Stage 0 means tokenizin
/// And assignes variables
#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum Operand<'a> {
    Number(f64),
    Variable(&'a str),
    Function(Function),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

impl Operator {
    fn precedence(&self) -> u8 {
        match self {
            Operator::Uplus | Operator::Uminus => 3,
            Operator::Power => 4,
            Operator::Multiply | Operator::Divide | Operator::Modulus => 2,
            Operator::Plus | Operator::Minus => 1,
        }
    }
}

impl PartialOrd for Operator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.precedence().cmp(&other.precedence()))
    }
}

impl Ord for Operator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.precedence().cmp(&other.precedence())
    }
}

#[derive(Debug, Clone)]
pub enum Token<'a> {
    Operand(Operand<'a>),
    Operator(Operator),
    ParenL,
    ParenR,
}

impl<'a> Token<'a> {
    pub fn is_integer(&self) -> bool {
        match &self {
            Token::Operand(_) => true,
            _ => false,
        }
    }
    pub fn is_operator(&self) -> bool {
        match &self {
            Token::Operator(_) => true,
            _ => false,
        }
    }
    pub fn is_paren_l(&self) -> bool {
        match &self {
            Token::ParenL => true,
            _ => false,
        }
    }
    pub fn is_paren_r(&self) -> bool {
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
    raw: String,
    pub structurized: Option<Vec<Token<'a>>>,
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
            raw: raw.to_owned(),
            structurized: None,
        }
    }

    // This function uses to fill variables into expression
    // The syntax is
    // x + y + x | 2 4
    // this should become
    // 2 + 4 + 2
    // so x = 2, y = 4
    fn replace_variables(&mut self) -> Result<(), Error> {
        let mut new_string = self.raw.to_owned();
        dbg!(&self.raw);
        if self.raw.contains("|") {
            let mut parts = self.raw.splitn(2, "|");
            let expr_part = parts.next().unwrap().trim();
            new_string = expr_part.to_owned();
            let values_part = parts.next().unwrap().trim();

            let mut var_names: Vec<&str> = expr_part
                .split_whitespace()
                .filter(|x| x.chars().all(|c| c.is_alphabetic()))
                .collect();

            let values: Vec<f64> = values_part
                .split_whitespace()
                .map(|s| s.parse::<f64>())
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| Err::<(), Error>(Error::InvalidVariable(0)))
                .unwrap();

            // if var_names.len() != values.len() {
            //     return Err(Error::InvalidVariable(0));
            // }

            let mut vars = HashMap::new();
            for (var, val) in var_names.iter().zip(values.iter()) {
                vars.insert(*var, Operand::Number(*val));
            }

            for (var, val) in vars.iter() {
                if let Operand::Number(num) = val {
                    new_string = new_string.replace(var, &num.to_string());
                }
            }
        }
        dbg!(&new_string);
        self.raw = new_string;
        Ok(())
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

            if !string.is_empty() {
                string.push_str(" ");
            }
            string.push_str(&token_str);
        }

        string
    }

    pub fn tokenize(&mut self) -> Result<(), Error> {
        if let Err(e) = self.replace_variables() {
            return Err(e);
        }

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
