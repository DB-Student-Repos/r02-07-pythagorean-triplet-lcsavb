use std::collections::HashSet;
use num::integer::gcd;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut triplets = HashSet::new();
    // Euclid's formula
    // a = m^2 - n^2, b = 2mn, c = m^2 + n^2
    // m and n must have different parity
    // the greatest common divisor of m and n must be 1


    // find squares find  a and b, not m and n. That is the issue.
    find_squares(sum as u32)
        .iter()
        .for_each(|&(m, n)| {
            if have_different_parity(m, n) && are_coprime(m as u32, n as u32) {
                let a = m*m - n*n;
                let b = 2*m*n;
                let c = m*m + n*n;

                let mut triplet = [a, b, c];
                triplet.sort_unstable();
                triplets.insert(triplet);
            }
        });

    triplets


}



fn are_coprime(a: u32, b: u32) -> bool {
    gcd(a, b) == 1
}

fn have_different_parity(a: u32, b: u32) -> bool {
    a % 2 != b % 2
}

// I think the problem is here
pub fn find_squares(sum: u32) -> Vec<(u32, u32)> {
    let mut results = Vec::new();

    for a in 1..=sum {
        if a * a > sum { 
            break; // Early termination
        }
        let b_squared = sum - a * a;
        let b = (b_squared as f64).sqrt() as u32;

        if b * b == b_squared {
            results.push((a, b));
        }
    }

    results
}