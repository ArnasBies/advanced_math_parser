#![allow(dead_code)]

#[derive(Debug)]
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
    I,
    Z,
    Digit(i64),
}

impl Operations{
    pub fn tokenize(expression: String) -> Option<Vec<Operations>>{
        let mut result: Vec<Operations> = vec![];
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
                for c in expression.chars().skip(i){
                    if c.is_numeric(){
                        number.push(c);
                        number_length += 1;
                    } else {
                        break;
                    }
                } 

                result.push(Operations::Digit(number.parse::<i64>().unwrap()));
                number = String::new();
            } else {
                result.push(match symbol {
                    'x' | 'X' => Operations::X,
                    'i' | 'I' => Operations::I,
                    'z' | 'Z' => Operations::Z,
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
        }

        println!("{:?}", result);

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
        //have to check whether there are no * / % ^ + - ( and * / % ^ ) after each other
        for symbol in tokenized_expression{

        }

        return true;
    }
}
