mod parser;
mod query;

fn main(){
    let mut expression: String = String::new();
    parser::Operations::tokenize("-cos(35) * arccos(234234) + log(23)".to_string());

    loop{

        //receives input and checks whether or not it was received properly
        match std::io::stdin().read_line(&mut expression){
            Err(input_error) => {
                println!("Your input was not received due to error: {}", input_error)
            },
            _ => {}
        }

        
    }
}
