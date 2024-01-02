#![allow(dead_code)]
use crate::parser::Operations;

pub struct Query{
    expression: Vec<Operations>,
    x: f64,
    z: f64,
    k: f64,
}

impl Query{
    pub fn new(expression: String) -> Option<Query>{
        match Operations::tokenize(expression){
            Some(x) => {
                return Some(Query{
                    expression: x,
                    x: 0.0,
                    z: 0.0,
                    k: 0.0
                })
            },
            None => return None,
        }
    }

    pub fn set_x(&mut self, new: f64){
        self.x = new;
    }

    pub fn set_z(&mut self, new: f64){
        self.z = new;
    }

    pub fn set_k(&mut self, new: f64){
        self.k = new;
    }

    pub fn evaluate(&self) -> Option<f64>{
        return self.calculate(&mut self.expression.clone())
    }

    fn calculate(&self, tokenized_expression: &mut Vec<Operations>) -> Option<f64>{
        use Operations::*;
        while tokenized_expression.contains(&LParen){
            let l_parenthesis_index = tokenized_expression.iter().position(|x| x == &LParen).unwrap();
            let r_parenthesis_index = Query::find_right_parenthesis(&tokenized_expression, l_parenthesis_index); 

            let number = match self.calculate(&mut (tokenized_expression[l_parenthesis_index + 1..r_parenthesis_index]).to_vec()){
                Some(x) => x,
                None => return None
            };

            tokenized_expression.drain(l_parenthesis_index..=r_parenthesis_index);
            tokenized_expression.insert(l_parenthesis_index, Operations::Digit(number));
        }        

        for i in 0..tokenized_expression.len() {
            match tokenized_expression.get(i).unwrap(){
                &X => {let _ = std::mem::replace(&mut tokenized_expression[i], Digit(self.x));},
                &Z => {let _ = std::mem::replace(&mut tokenized_expression[i], Digit(self.z));},
                &K => {let _ = std::mem::replace(&mut tokenized_expression[i], Digit(self.k));},
                _ => {},
            }
        }

        //collapses the functions
        let mut index = 0;
        while tokenized_expression.len() > index{
            match tokenized_expression.get(index).unwrap(){
                Log | Sin | Cos | Tan | ArcSin | ArcCos | ArcTan => {
                    let new_number = match Query::collapse_function(tokenized_expression.get(index), tokenized_expression.get(index + 1)){
                        Some(x) => x,
                        None => return None
                    };

                    tokenized_expression.drain(index..=index+1);
                    tokenized_expression.insert(index, Digit(new_number));
                },
                _ => index += 1
            } 
        }

        //Exponent
        index = 1;
        while tokenized_expression.len() > index{
            if tokenized_expression.get(index).unwrap() == &Exponent{
                let new_number = match Query::collapse(tokenized_expression.get(index - 1), tokenized_expression.get(index), tokenized_expression.get(index + 1)){
                    Some(x) => x,
                    None => return None,
                };

                tokenized_expression.drain(index-1..=index+1);
                tokenized_expression.insert(index - 1, Digit(new_number));
            } else {
                index += 1;
            }
        }

        // / * %
        index = 1;
        while tokenized_expression.len() > index{
            match tokenized_expression.get(index).unwrap(){
                Remainder | Division | Multiplication => {
                    let new_number = match Query::collapse(tokenized_expression.get(index - 1), tokenized_expression.get(index), tokenized_expression.get(index + 1)){
                        Some(x) => x,
                        None => return None,
                    };

                    tokenized_expression.drain(index-1..=index+1);
                    tokenized_expression.insert(index - 1, Digit(new_number));
                    
                },
                _ => {index += 1},
            }
        }

        // - +
        index = 1;
        while tokenized_expression.len() > index{
            match tokenized_expression.get(index).unwrap(){
                Addition | Subtraction => {
                    let new_number = match Query::collapse(tokenized_expression.get(index - 1), tokenized_expression.get(index), tokenized_expression.get(index + 1)){
                        Some(x) => x,
                        None => return None,
                    };

                    tokenized_expression.drain(index-1..=index+1);
                    tokenized_expression.insert(index - 1, Digit(new_number));
                    
                },
                _ => {index += 1},
            }
        }
        
        match tokenized_expression.get(0).unwrap(){
            Digit(x) => return Some(*x),
            _ => return None
        }
    }

    fn find_right_parenthesis(tokenized_expression: & Vec<Operations>, l_parenthesis_index: usize) -> usize{
        let (mut to_skip, mut i) = (0, l_parenthesis_index + 1);

        for symbol in tokenized_expression[l_parenthesis_index+1..].iter(){
            if symbol == &Operations::LParen{
                to_skip += 1;
            } else if symbol == &Operations::RParen && to_skip != 0{
                to_skip -= 1;
            } else if symbol == &Operations::RParen && to_skip == 0{
                return i;
            }

            i += 1;
        }
        return 0;
    }

    fn collapse(l_number: Option<&Operations>, operation: Option<&Operations>, r_number: Option<&Operations>) -> Option<f64>{
        use Operations::*;
        if l_number.is_none() || r_number.is_none() || operation.is_none(){
            return None
        }

        match (l_number.unwrap(), r_number.unwrap()){
            (Digit(l), Digit(r)) => {
                return Some(match operation.unwrap() {
                    Exponent => l.powf(*r),
                    Multiplication => l * r,
                    Division => l / r,
                    Remainder => l % r,
                    Addition => l + r,
                    Subtraction => l - r,
                    _ => return None,
                })
            },
            _=> None
        }
    }

    fn collapse_function(function: Option<&Operations>, number: Option<&Operations>) -> Option<f64>{
        use Operations::*;

        if function.is_none() || number.is_none() {
            return None
        }


        match (function.clone().unwrap(), number.unwrap()){
            (Log | Sin | Cos | Tan | ArcSin | ArcCos | ArcTan, Digit(x)) => {
                return Some(match function.unwrap(){
                    Log => x.log10(),
                    Sin => x.sin(),
                    Cos => x.cos(),
                    Tan => x.tan(),
                    ArcSin => x.asin(),
                    ArcCos => x.acos(),
                    ArcTan => x.atan(),
                    _ => return None,
                })
            },
            _ => return None
        }
    }
}
