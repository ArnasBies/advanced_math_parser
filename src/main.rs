mod parser;
mod query;

fn main(){
    let mut expression: String; 
    let mut math_expression: query::Query;

    loop{
        //receives input and checks whether or not it was received properly
        expression = String::new();
        match std::io::stdin().read_line(&mut expression){
            Err(input_error) => {
                println!("Your input was not received due to error: {}", input_error)
            },
            _ => {}
        }

        math_expression = match query::Query::new(expression){
            Some(x) => x,
            None => {
                println!("Invalid expression\n");
                continue;
            }
        };

        match math_expression.evaluate(){
            Some(x) => println!("= {}\n\n", x),
            None => println!("Invalid expression\n\n")
        }

    }
}
