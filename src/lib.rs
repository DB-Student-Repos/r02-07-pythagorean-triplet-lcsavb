use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut triplets = HashSet::new();
    // Euclid's formula
    // a = m^2 - n^2, b = 2mn, c = m^2 + n^2
    // m and n must have different parity
    // the greatest common divisor of m and n must be 1
    


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


