pub fn square_of_sum(n: u32) -> u32 {
    let mut vector: Vec<u32> = Vec::new();
    for  number in 1..(n+1) {
        vector.push(number)
    }
    let sum: u32 = vector.iter().sum();
    let square = u32::pow(sum,2);
    square
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut vector: Vec<u32> = Vec::new();
    for  number in 1..(n+1) {
        vector.push(u32::pow(number,2))
    }
    let sum: u32 = vector.iter().sum();
    sum
}

pub fn difference(n: u32) -> u32 {
    let square_of_sum = square_of_sum(n);
    let sum_of_squares = sum_of_squares(n);
    let difference = square_of_sum - sum_of_squares;
    difference
}