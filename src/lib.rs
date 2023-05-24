#[derive(Debug, Clone, Copy)]
enum Token<T> {
    Number(T),
    Operator(OPSymbol),
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum OpAssocation {
    LEFT,
    RIGHT,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum OPSymbol {
    ADD,
    SUB,
    MUL,
    DIV,
    EXP,
    LeftParen,
    RightParen,
}

impl OPSymbol {
    //Convert char to OP
    fn value(c: char) -> Option<OPSymbol> {
        use OPSymbol::*;
        match c {
            '+' => Some(ADD),
            '-' => Some(SUB),
            '*' => Some(MUL),
            '/' => Some(DIV),
            '^' => Some(EXP),
            '(' => Some(LeftParen),
            ')' => Some(RightParen),
            _ => None,
        }
    }

    fn get_precedence(self) -> u8 {
        use OPSymbol::*;
        match self {
            ADD | SUB => 2,
            MUL | DIV => 3,
            EXP => 4,
            LeftParen | RightParen => 0,
        }
    }

    fn get_association(self) -> OpAssocation {
        use OPSymbol::*;
        match self {
            ADD | SUB | MUL | DIV => OpAssocation::LEFT,
            EXP | LeftParen | RightParen => OpAssocation::RIGHT,
        }
    }

    //evaluate OpSymbol and perform operation
    fn eval(i1: f64, i2: f64, op: &OPSymbol) -> Result<Token<f64>, &'static str> {
        use Token::Number;
        match op {
            OPSymbol::ADD => Ok(Number(i1 + i2)),
            OPSymbol::SUB => Ok(Number(i1 - i2)),
            OPSymbol::MUL => Ok(Number(i1 * i2)),
            OPSymbol::DIV => {
                if i2 == 0.0 {
                    return Err("Can't divide by 0!");
                } else {
                    Ok(Number(i1 / i2))
                }
            }
            OPSymbol::EXP => Ok(Number(i1.powf(i2))),
            _ => Err("Can't evaluate invalid symbol"),
        }
    }
}

/// Parses a given expression string and returns a result as an f64
/// If any parsing/calculation errors occur returns None
/// Uses the shunting yard algorithm to parse the inputs into reverse polish notation
/// Handles basic Addition, Subtraction, Multiplication, Division, Exponents, the unary - operator
/// ```
/// let result = match calculator::parse("2+2") {
///   Ok(x) => x,
///   Err(e) => panic!("{e}")
/// };
/// assert_eq!(result, 4.0);
///
/// let result = match calculator::parse("2*(1+3)^2") {
///   Ok(x) => x,
///   Err(e) => panic!("{e}")
/// };
/// assert_eq!(result, 32.0);
/// ```
pub fn parse(input: &str) -> Result<f64, &str> {
    use OPSymbol::*;
    //immedieatly die if we find alpha characters
    if input.contains(|c: char| c.is_ascii_alphabetic()) {
        return Err("Found Invalid Input!");
    }

    //get rid of spaces
    //use map() and closure to combine steps?
    let mut trimmed_input = String::new();
    for s in input.split_whitespace() {
        trimmed_input.push_str(s);
    }
    //convert to iterator since String and &str can't be iterated on
    let char_iter = trimmed_input.chars().into_iter();

    //manage index and offset of &str
    let mut index = 0;
    let mut offset = 0;
    let mut prev_val = '`';
    //token vector that will be passed to obtain final f64 result
    let mut tokens: Vec<Token<f64>> = Vec::new();
    let mut op_stack: Vec<OPSymbol> = Vec::new();

    for c in char_iter {
        //handle unary negative operator
        //Better way to hanlde this?
        if (c.is_ascii_digit() || c == '.')
            || (index == 0 && c == '-')
            || (c == '-' && !prev_val.is_ascii_digit() && prev_val != '`')
        {
            index += 1;
            continue;
        } else {
            //found number boundry
            if offset != index {
                tokens.push(Token::Number(match trimmed_input[offset..index].parse() {
                    Ok(x) => x,
                    Err(_) => {
                        return Err("Found Symbol, Expected Number");
                    }
                }));
            }
            //convert char to OpSymbol
            let op1 = match OPSymbol::value(c) {
                Some(x) => x,
                None => {
                    return Err("Found invalid symbol");
                }
            };
            //process logic for for op token
            //unconditionally add ( to the opstack
            if op1 == OPSymbol::LeftParen {
                op_stack.push(op1);
            //hanlde )
            } else if op1 == RightParen && op_stack.len() > 0 {
                //pop opstack until ( is found
                while op_stack.len() > 0 {
                    let op2 = op_stack[op_stack.len() - 1];
                    if op2 != LeftParen {
                        tokens.push(Token::Operator(match op_stack.pop() {
                            Some(x) => x,
                            None => {
                                return Err("Found invalider OP token");
                            }
                        }));
                    } else {
                        //pop the ( and leave the loop
                        op_stack.pop();
                        break;
                    }
                    //found a ) but no matching (
                    if op_stack.len() == 0 {
                        return Err("Found mismatched ()");
                    }
                }
            //handle other operators
            } else {
                //if op stack is empty just add op
                if op_stack.len() == 0 {
                    op_stack.push(op1);
                } else {
                    while op_stack.len() > 0 {
                        let op2 = op_stack[op_stack.len() - 1];
                        if op2 != LeftParen
                            && (op2.get_precedence() > op1.get_precedence()
                                || (op1.get_precedence() == op2.get_precedence()
                                    && op1.get_association() == OpAssocation::LEFT))
                        {
                            tokens.push(Token::Operator(match op_stack.pop() {
                                Some(x) => x,
                                None => {
                                    return Err("Failed to push operator to token output");
                                }
                            }));
                        } else {
                            break;
                        }
                    }
                    op_stack.push(op1);
                }
            }
        }
        prev_val = c;
        index += 1;
        offset = index;
    }
    //if end of input was not a ) or some other symbol offset to end of input must be a number. Push this number to output vector
    if offset < trimmed_input.len() {
        tokens.push(Token::Number(match trimmed_input[offset..].parse() {
            Ok(x) => x,
            Err(e) => {
                eprintln!("{e}");
                return Err("Failed to parse number");
            }
        }));
    }
    //finsished iterating over string push remaining op symbols on to output
    if op_stack.len() > 0 {
        while op_stack.len() > 0 {
            tokens.push(Token::Operator(match op_stack.pop() {
                Some(x) => {
                    if x == LeftParen {
                        return Err("Found unclosed (");
                    } else {
                        x
                    }
                }
                None => continue,
            }));
        }
    }

    match get_result(tokens) {
        Ok(x) => Ok(x),
        Err(e) => Err(e),
    }
}

// Convert vector of Tokens to a final result as a f64 number
fn get_result(mut tokens: Vec<Token<f64>>) -> Result<f64, &'static str> {
    use Token::{Number, Operator};
    loop {
        let n = tokens.len();
        if n == 0 {
            return Err("");
        } else if let [Number(a)] = tokens[..] {
            return Ok(a);
        } else if n < 3 {
            return Err("Syntax Error");
        } else {
            let mut index = 0;
            let mut temp_result: Token<f64> = Number(0.0);
            for _t in &tokens {
                //find first instance of an OP symbol
                if let Operator(_op) = &tokens[index] {
                    //Take the 2 Numbers precedding the symbol and evaluate
                    if let [Number(a), Number(b), Operator(op)] = tokens[index - 2..=index] {
                        match OPSymbol::eval(a, b, &op) {
                            Ok(c) => {
                                temp_result = c;
                                break;
                            }
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    }
                }
                index += 1;
            }
            //replace evaluated tokens with intermediate result
            let t: Vec<Token<f64>> = Vec::from([temp_result]);
            tokens.splice(index - 2..=index, t);
        }
    }
}
