pub fn reverse(input: &str) -> String {    
    let reverse_input = input.chars().rev().collect::<String>();
    println!("input: {:?} and reverse_input: {:?}", input, reverse_input);
    reverse_input
}

// Cleaner community solution
pub fn reverse(data: &str) -> String {
    data.chars().rev().collect()
}