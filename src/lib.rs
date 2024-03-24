use std::collections::HashSet;
use num::integer::gcd;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut triplets = HashSet::new();
    // Euclid's formula
    // a = m^2 - n^2, b = 2mn, c = m^2 + n^2
    

    // O(n²) i will try to optimize it
    for m in 2..sum {
        for n in 1..m {
            
                let a = m * m - n * n;
                let b = 2 * m * n;
                let c = m * m + n * n;
                if a + b + c == sum {
                    let mut triplet = [a, b, c];
                    triplet.sort();
                    triplets.insert(triplet);
                }
            
        }
    }
    triplets
} 


// the previous error that only PRIMITIVE TRIPLETS have to
// respect the following conditions:
// m and n must have different parity
// the greatest common divisor of m and n must be 1
