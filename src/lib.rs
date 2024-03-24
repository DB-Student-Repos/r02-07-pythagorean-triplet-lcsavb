use std::collections::HashSet;

// rearranged pythagorean theorem method
// Big O: O(n)

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut triplets = HashSet::new();

    
    let mut a = 1;
    loop {
        // Loop through possible values of 'a' (shortest side)
        // it will certainly be always less than the sum divided by 3
        if a > sum / 3 {
            break;
        }

        // Calculate the remaining sum for sides 'b' and 'c'
        let sum_bc = sum - a;

        // Given a Pythagorean triplet (a, b, c), where a, b, and c are the lengths 
        // of the sides of a right-angled triangle, the Pythagorean theorem states that a² + b² = c². 
        //
        // Also, given that the sum of the sides of the triangle is a constant (let's call it `sum`), 
        // we have a + b + c = `sum`, which can be rearranged to b + c = `sum - a` (let's call this `sum_bc`).
        //
        // Now, if we square both sides of the equation b + c = `sum_bc`, we get b² + 2bc + c² = (`sum_bc`)².
        //
        // Substituting a² + b² = c² into the equation, we get a² + 2bc = (`sum_bc`)² - a².
        //
        // Rearranging for b, we get b = ((`sum_bc`)² - a²) / 2bc.
        //
        //- The denominator is 2bc, which is calculated as `2 * sum_bc`.
        //- The numerator is (`sum_bc`)² - a², which is calculated as `sum_bc * sum_bc - a * a`.

        let denominator = 2 * sum_bc;
        let numerator = sum_bc * sum_bc - a * a;

        // If 'b' is an integer
        if numerator % denominator == 0 {
            let b = numerator / denominator;

            // Ensure 'a' is the shortest side
            if a < b {
                let c = sum - a - b;

                // Store the triplet
                triplets.insert([a, b, c]);
            }
        }

        a += 1;
    }

    triplets
}


