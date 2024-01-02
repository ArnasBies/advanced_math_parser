#![allow(dead_code)]

#[derive(Debug, PartialEq, Clone)]
pub enum Operations{
    RParen,
    LParen,
    Exponent,
    Multiplication,
    Division,
    Remainder,
    Addition,
    Subtraction,
    X,
    Z,
    K,
    Digit(f64),
    Log,
    Sin,
    ArcSin,
    Cos,
    ArcCos,
    Tan,
    ArcTan,
}

impl Operations{
    pub fn tokenize(expression: String) -> Option<Vec<Operations>>{
        let mut result: Vec<Operations> = vec![];
        let mut expression = expression.to_lowercase();
        expression.retain(|c| !c.is_whitespace());
        let mut number: String = String::new();
        let mut number_length: u16 = 0;

        for (i, symbol) in expression.chars().enumerate(){
            //if it's a number it gets following digits and skips up till the end of the number
            //skips when encountering a finished number
            if number_length > 0{
                number_length -= 1;
                continue;
            }

            if symbol.is_numeric(){
                number.push(symbol);
                for c in expression.chars().skip(i+1){
                    if c.is_numeric() || c == '.'{
                        number.push(c);
                        number_length += 1;
                    } else {
                        break;
                    }
                } 

                result.push(Operations::Digit(number.parse::<f64>().unwrap()));
                number = String::new();
                continue;
            } else if symbol.is_alphabetic(){
                match expression.chars().collect::<String>().as_str().get(i..i+3) {
                    Some(x) => {
                        match x {
                            "sin" => result.push(Operations::Sin),
                            "cos" => result.push(Operations::Cos),
                            "tan" => result.push(Operations::Tan),
                            "log" => result.push(Operations::Log),
                            _ => {}
                        };
                        match x {
                            "sin" | "cos" | "tan" | "log" => {
                                number_length += 2;
                                continue;
                            }
                            _ => {}
                        }
                    },
                    None => {},
                }; 
                match expression.chars().collect::<String>().as_str().get(i..i+6) {
                    Some(x) => {
                        match x {
                            "arcsin" => result.push(Operations::ArcSin),
                            "arccos" => result.push(Operations::ArcCos),
                            "arctan" => result.push(Operations::ArcTan),
                            _ => {}
                        };
                        match x {
                            "arcsin" | "arccos" | "arctan" => {
                                number_length += 5;
                                continue;
                            },
                            _ => {}
                        }
                    },
                    None => {},
                }; 
            } 
            result.push(match symbol {
                'x' => Operations::X,
                'z' => Operations::Z,
                'k' => Operations::K,
                ')' => Operations::RParen,
                '(' => Operations::LParen,
                '^' => Operations::Exponent,
                '*' => Operations::Multiplication,
                '/' => Operations::Division,
                '%' => Operations::Remainder,
                '+' => Operations::Addition,
                '-' => Operations::Subtraction,
                _ => return None,
            });
        }

        if Operations::pre_check(&mut result) {
            return Some(result)
        } else {
            return None
        }
    }

    fn pre_check(tokenized_expression: &mut Vec<Operations>) -> bool{
        return Operations::valid_parenthesis(tokenized_expression) && Operations::symbol_order(tokenized_expression)
    }

    fn valid_parenthesis(tokenized_expression: &Vec<Operations>) -> bool{
        let mut pair_count: u16 = 0;

        for symbol in tokenized_expression{
            match symbol{
                Operations::LParen => {
                    pair_count += 1;
                },
                Operations::RParen => {
                    if pair_count > 0 {
                        pair_count -= 1;
                    } else{
                        return false;
                    }
                },
                _ => {}
            }
        }
        return true;
    }

    fn symbol_order(tokenized_expression: &mut Vec<Operations>) -> bool{
        match *tokenized_expression.get(0).unwrap(){
            Operations::RParen | Operations::Exponent | Operations::Multiplication | Operations::Division | Operations::Remainder => {return false;},
            Operations::Addition => {tokenized_expression.remove(0);},
            _ => {}, 
        };
         
        //have to merge + and minus where there are several in a row,
        //have to add multiplication between digits and variables(x, z, i)
        //have to check whether there are no * / % ^ + - ( Log Sin Cos Tan and Arc versions and * / % ^ ) after each other

        //merges + and -
        let mut index = 0;
        for _ in 0..tokenized_expression.len() {
            let operation = tokenized_expression.get(index).unwrap();
            match operation {
                Operations::Addition | Operations::Subtraction => {
                    match tokenized_expression.get(index + 1) {
                        Some(x) => {
                            match x {
                                Operations::Addition | Operations::Subtraction => {
                                    if x == operation{
                                        tokenized_expression.drain(index..=index+1);
                                        tokenized_expression.insert(index, Operations::Addition);
                                    } else {
                                        tokenized_expression.drain(index..=index+1);
                                        tokenized_expression.insert(index, Operations::Subtraction);
                                    }
                                    continue;
                                },
                                _ => {index += 1},
                            }
                        },
                        None => return false,
                    }
                },

                _ => {index += 1},
            }
        }

        //adds multiplication between x and character
        index = 0;
        for i in 0..tokenized_expression.len() {
            match tokenized_expression.get(i).unwrap() {
                Operations::Digit(_) => {
                    match tokenized_expression.get(i+1){
                        Some(x) => {
                            if x == &Operations::X || x == &Operations::Z || x == &Operations::K{
                                tokenized_expression.insert(index + i + 1, Operations::Multiplication);
                            }
                        },
                        None => {}
                    }
                },
                _ => {}
            }
            
        }
        
        //have to check whether there are no * / % ^ + - ( Log Sin Cos Tan and Arc versions and * / % ^ ) after each other
        let first_table = vec![Operations::Multiplication, Operations::Division, Operations::Remainder, Operations::Exponent, Operations::Addition, Operations::Subtraction, Operations::LParen, Operations::Log, Operations::Sin, Operations::Cos, Operations::Tan, Operations::ArcSin, Operations::ArcCos, Operations::ArcTan];
        let second_table = vec![Operations::Multiplication, Operations::Exponent, Operations::Division, Operations::Remainder, Operations::RParen];
        for i in 0..tokenized_expression.len()-1{
            if first_table.contains(tokenized_expression.get(i).unwrap()) && second_table.contains(tokenized_expression.get(i+1).unwrap()){
                return false
            }    
        }

        for i in 0..tokenized_expression.len() - 1{
            match tokenized_expression.get(i).unwrap() {
                Operations::X | Operations::Z | Operations::K => {
                    match tokenized_expression.get(i + 1).unwrap() {
                        Operations::Digit(_) => return false,
                        _ => {}
                    }
                },
                _ => {}
            }
        }


        return true;
    }
}
