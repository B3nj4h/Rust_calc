use std::env::{ args, Args};

fn main(){
    let mut args: Args = args();

    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();

    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();
    let result = operation(operator, first_number, second_number);
    
    println!("{:?}", output(operator, first_number, second_number, result));

}

fn operation(operator: char, first: f32, second: f32) -> f32 {
    match operator {
        '+' => first+second,
        '-' => first - second,
        '/' => first / second,
        '*' | 'x' | 'X' => first * second,
        _ => panic!("Invalid operator")
    }
}

fn output(operator: char, first: f32, second: f32, result: f32) -> String {
    format!("{} {} {} = {}", first, operator, second, result)   
}