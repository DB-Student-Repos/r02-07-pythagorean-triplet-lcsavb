use std::collections::HashSet;
use num::integer::gcd;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut triplets = HashSet::new();
    // Euclid's formula
    // a = m^2 - n^2, b = 2mn, sum = m^2 + n^2
    // m and n must have different parity
    // the greatest common divisor of m and n must be 1



    find_squares(sum as u32)
        .iter()
        .for_each(|&(a, b)| {

        });

    triplets


}



fn are_coprime(m: u32, n: u32) -> bool {
    gcd(a, b) == 1
}

fn have_different_parity(m: u32, n: u32) -> bool {
    a % 2 != b % 2
}


pub fn find_squares(sum: u32) -> Vec<(u32, u32)> {
    unimplemented!()
}