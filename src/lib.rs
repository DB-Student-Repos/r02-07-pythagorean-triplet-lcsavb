use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut triplets = HashSet::new();
    // Euclid's formula
    // a = m^2 - n^2, b = 2mn, c = m^2 + n^2
    // m and n must have different parity
    // the greatest common divisor of m and n must be 1

    find_squares(sum as u32)
        .iter()
        .for_each(|&(m, n)| {
            if have_different_parity(m, n) && are_coprime(m as u32, n as u32, sum) {
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

fn gcd(mut a: u32, mut b: u32, mut c: u32) -> u32 {
    let mut d = a;

    while b != 0 {
        d = b;
        b = a % b;
        a = d;
    }

    while c != 0 {
        d = c;
        c = a % c;
        a = d;
    }

    d
}

fn are_coprime(a: u32, b: u32, c: u32) -> bool {
    gcd(a, b, c) == 1
}

fn have_different_parity(a: u32, b: u32) -> bool {
    a % 2 != b % 2
}

// I think the problem is here
pub fn find_squares(sum: u32) -> Vec<(u32, u32)> {
    let mut results = Vec::new();

    let limit = (sum as f64).sqrt() as u32;

    for a in 1..limit {
        for b in a+1..=limit {
            if a*a + b*b == sum {
                results.push((a, b));
            }
        }
    }

    results
}