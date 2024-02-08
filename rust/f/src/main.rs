use std::io;

fn convert_to_int(input: &String) -> i32 {
    let x = input.trim().parse::<i32>().unwrap();
    x
}

fn main() {
    let mut sum = 0;
    let mut input_value = String::new();

    io::stdin()
        .read_line(&mut input_value)
        .expect("Some error occured");

    let mut value = convert_to_int(&input_value);

    while value != 0 {
        let rest = value % 10;
        sum += rest;
        value = value / 10;
    }

    println!("{}", sum);
}
