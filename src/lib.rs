use std::collections::HashSet;
use num::integer::gcd;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut triplets = HashSet::new();

    // Precompute squares
    let squares: Vec<u32> = (0..=sum).map(|x| x * x).collect();


    for m in 2..sum {
        for n in 1..m {
            if (m - n) % 2 == 1 && gcd(m, n) == 1 {
                for k in 1..=sum / (2 * m * m) {
                    let a = k * (squares[m as usize] - squares[n as usize]);
                    let b = k * 2 * m * n;
                    let c = k * (squares[m as usize] + squares[n as usize]);
                    if a + b + c == sum {
                        let mut triplet = [a, b, c];
                        triplet.sort();
                        triplets.insert(triplet);
                    }
                }
            }
        }
    }

    triplets
}



// the previous error that only PRIMITIVE TRIPLETS have to
// respect the following conditions:
// m and n must have different parity
// the greatest common divisor of m and n must be 1
// moreover I forgot to add the multiplication factor k
