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
        .for_each(|&(m, n)| {

        });

    triplets


}



fn are_coprime(m: u32, n: u32) -> bool {
    gcd(m, n) == 1
}

fn have_different_parity(m: u32, n: u32) -> bool {
    m % 2 != n % 2
}


pub fn find_squares(sum: u32) -> Vec<(u32, u32)> {
    let mut squares = Vec::new();

    for m in 1..sum {
        for n in 1..sum {
            if m.pow(2) + n.pow(2) == sum {
                squares.push((m, n));
            }
        }
    }
    squares
}