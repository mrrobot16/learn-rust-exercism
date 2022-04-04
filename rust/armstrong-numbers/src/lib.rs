pub fn is_armstrong_number(number: u32) -> bool {
    print_space();
    
    let armstrong_number = number == calculate_armstrong_number(number);
    
    println!("armstrong_number: {}", armstrong_number);
    print_space();
    
    armstrong_number
}

pub fn calculate_armstrong_number(num: u32) -> u32 {
    let number_string = num.to_string();
    let numbers_list_chars =  number_string.chars();
    println!("numbers_list_chars: {:?}", numbers_list_chars);
    
    let numbers_list = numbers_list_chars
        .map(|d| d.to_digit(10).unwrap())
        .collect::<Vec<_>>()
        .into_iter();
    println!("numbers_list: {:?}", numbers_list);
    
    let number_digits = numbers_list.len() as u32;
    println!("number_digits: {}", number_digits);
    
    let mut counter = 0;
    for number in numbers_list {
        println!("number_in_list: {}", number);
        let exponent = pow(number, number_digits);
        counter += exponent
    }
    println!("counter: {}", counter);
    counter
}

fn pow(base: u32, exponent: u32) -> u32 {
    let pow = u32::pow(base, exponent);
    pow
}

fn print_space() {
    println!("                          ");
    println!("                          ");
    println!("                          ");
    println!("                          ");
    println!("                          ");
    println!("                          ");
    println!("                          ");
    println!("                          ");
    println!("*************************");
    println!("*************************");
}