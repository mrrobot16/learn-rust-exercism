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
    // unimplemented!("sum of squares of 1...{}", n);
    3
}

pub fn difference(n: u32) -> u32 {
    // unimplemented!(
    //     "difference between square of sum of 1...{n} and sum of squares of 1...{n}",
    //     n = n,
    // );
    3
}

// pub fn total() -> u64 {
    // let total_sum_of_grains_in_board = build_calculated_grains_board(64).iter().map(|x| x).sum();
    // total_sum_of_grains_in_board
// }