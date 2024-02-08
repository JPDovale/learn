use std::io;

fn convert_to_int(input: &String) -> i32 {
    let x = input.trim().parse::<i32>().unwrap();
    x
}

fn main() {
    let mut number1_input = String::new();
    io::stdin()
        .read_line(&mut number1_input)
        .expect("Some error occured");

    let mut number2_input = String::new();
    io::stdin()
        .read_line(&mut number2_input)
        .expect("Some error occured");

    let number1 = convert_to_int(&number1_input);
    let number2 = convert_to_int(&number2_input);

    if number1 > number2 {
        println!("First number is bigger: {} > {}", number1, number2);
    } else {
        println!("Second number is bigger: {} < {}", number1, number2);
    }
}
