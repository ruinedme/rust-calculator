#[derive(Debug)]
enum Token<T> {
    Number(T),
    Operator(OP),
}

#[derive(Debug, PartialEq)]
enum OpAssocation {
    LEFT,
    RIGHT,
}

#[derive(Debug, PartialEq)]
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
    fn value(c: char) -> Option<OP> {
        match c {
            '+' => Some(OP {
                op_symbol: OPSymbol::ADD,
                precedence: 2,
                op_association: OpAssocation::LEFT,
            }),
            '-' => Some(OP {
                op_symbol: OPSymbol::SUB,
                precedence: 2,
                op_association: OpAssocation::LEFT,
            }),
            '*' => Some(OP {
                op_symbol: OPSymbol::MUL,
                precedence: 3,
                op_association: OpAssocation::LEFT,
            }),
            '/' => Some(OP {
                op_symbol: OPSymbol::DIV,
                precedence: 3,
                op_association: OpAssocation::LEFT,
            }),
            '^' => Some(OP {
                op_symbol: OPSymbol::EXP,
                precedence: 4,
                op_association: OpAssocation::RIGHT,
            }),
            '(' => Some(OP {
                op_symbol: OPSymbol::LeftParen,
                precedence: 0,
                op_association: OpAssocation::RIGHT,
            }),
            ')' => Some(OP {
                op_symbol: OPSymbol::RightParen,
                precedence: 0,
                op_association: OpAssocation::RIGHT,
            }),
            _ => None,
        }
    }

    //evaluate OpSymbol and perform operation
    fn eval(i1: f64, i2: f64, op: &OPSymbol) -> Option<f64> {
        match op {
            OPSymbol::ADD => Some(i1 + i2),
            OPSymbol::SUB => Some(i1 - i2),
            OPSymbol::MUL => Some(i1 * i2),
            OPSymbol::DIV => {
                if i2 == 0.0 {
                    println!("Can't divide by 0!");
                    return None;
                } else {
                    Some(i1 / i2)
                }
            }
            OPSymbol::EXP => Some(i1.powf(i2)),
            _ => {
                println!("Can't evaluate invalid symbol");
                None
            }
        }
    }
}

#[derive(Debug)]
struct OP {
    op_symbol: OPSymbol,
    precedence: u8,
    op_association: OpAssocation,
}

/// Parses a given expression string and returns a result as an f64
/// If any parsing/calculation errors occur returns None
/// Uses the shunting yard algorithm to parse the inputs into reverse polish notation
/// Handles basic Addition, Subtraction, Multiplication, Division, Exponents, the unary - operator
/// ```
/// let result = match calculator::to_result(String::from("2+2")) {
///   Some(x) => x,
///   None => panic!("Test Failed")
/// };
/// assert_eq!(result, 4.0);
///
/// let result = match calculator::to_result(String::from("2*(1+3)^2")) {
///   Some(x) => x,
///   None => panic!("Test Failed")
/// };
/// assert_eq!(result, 32.0);
/// ```
pub fn to_result(input: String) -> Option<f64> {
    //immedieatly die if we find alpha characters
    if input.contains(|c: char| c.is_ascii_alphabetic()) {
        println!("Found Invalid Input!");
        return None;
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
    let mut op_stack: Vec<OP> = Vec::new();

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
                        println!("Found Symbol, Expected Number");
                        return None;
                    }
                }));
            }
            //convert char to OpSymbol
            let op1 = match OPSymbol::value(c) {
                Some(x) => x,
                None => {
                    println!("Found invalid symbol");
                    return None;
                }
            };
            //process logic for for op token
            //unconditionally add ( to the opstack
            if op1.op_symbol == OPSymbol::LeftParen {
                op_stack.push(op1);
            //hanlde )
            } else if op1.op_symbol == OPSymbol::RightParen && op_stack.len() > 0 {
                //pop opstack until ( is found
                while op_stack.len() > 0 {
                    let op2 = &op_stack[op_stack.len() - 1];
                    if op2.op_symbol != OPSymbol::LeftParen {
                        tokens.push(Token::Operator(match op_stack.pop() {
                            Some(x) => x,
                            None => {
                                println!("Found invalider OP token");
                                return None;
                            }
                        }));
                    } else {
                        //pop the ( and leave the loop
                        op_stack.pop();
                        break;
                    }
                    //found a ) but no matching (
                    if op_stack.len() == 0 {
                        println!("Found mismatched ()");
                        return None;
                    }
                }
            //handle other operators
            } else {
                //if op stack is empty just add op
                if op_stack.len() == 0 {
                    op_stack.push(op1);
                } else {
                    while op_stack.len() > 0 {
                        let op2 = &op_stack[op_stack.len() - 1];
                        if op2.op_symbol != OPSymbol::LeftParen
                            && (op2.precedence > op1.precedence
                                || (op1.precedence == op2.precedence
                                    && op1.op_association == OpAssocation::LEFT))
                        {
                            tokens.push(Token::Operator(match op_stack.pop() {
                                Some(x) => x,
                                None => {
                                    println!("Failed to push operator to token output");
                                    return None;
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
            Err(_) => {
                println!("Found Symbol, Expected Number");
                return None;
            }
        }));
    }
    //finsished iterating over string push remaining op symbols on to output
    if op_stack.len() > 0 {
        while op_stack.len() > 0 {
            tokens.push(Token::Operator(match op_stack.pop() {
                Some(x) => {
                    if x.op_symbol == OPSymbol::LeftParen {
                        println!("Found unclosed (");
                        return None;
                    } else {
                        x
                    }
                }
                None => continue,
            }));
        }
    }
    //missing symbol
    //probably breaks if unary symbols are ever implemented like !5
    if tokens.len() % 2 == 0 {
        println!("Invalid Expression");
        return None;
    }
    get_result(tokens)
}

/// Convert vector of Tokens to a final result as a f64 number
fn get_result(mut tokens: Vec<Token<f64>>) -> Option<f64> {
    let mut index = 0;
    while tokens.len() > 1 {
        //temp result
        let mut r: Token<f64> = Token::Number(0.0);
        for t in &tokens {
            match t {
                Token::Number(_x) => {
                    index += 1;
                    continue;
                }
                Token::Operator(x) => {
                    let i1 = match tokens[index - 2] {
                        Token::Number(n) => n,
                        _ => {
                            println!("Found OPSymbol, expected Number");
                            return None;
                        }
                    };
                    let i2 = match tokens[index - 1] {
                        Token::Number(n) => n,
                        _ => {
                            println!("Found OpSymbol, expected Number");
                            return None;
                        }
                    };
                    r = Token::Number(match OPSymbol::eval(i1, i2, &x.op_symbol) {
                        Some(x) => x,
                        _ => return None,
                    });
                    break;
                }
            }
        } //end for

        //can't borrow immutable and mutable in same scope so need to update tokens outside for loop
        //update tokens
        tokens.remove(index);
        tokens.remove(index - 1);
        tokens.remove(index - 2);
        tokens.insert(index - 2, r);
        //reset for next loop
        index = 0;
    } //end while
    match tokens[0] {
        Token::Number(x) => Some(x),
        _ => {
            println!("Expected Number, found Symbol");
            return None;
        }
    }
}
