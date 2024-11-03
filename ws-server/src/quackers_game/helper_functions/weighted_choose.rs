use rand::Rng;

/// Takes a tuple like the one that define the available duck colors.
/// Panics if given an empty array. 
pub fn weighted_choose<T: Copy, R: Rng>(options: &[(T, u32)], rng: &mut R) -> T {

    let total_weight: u32 = options.iter().map(|&(_, weight)| weight).sum();
    let mut random_value = rng.gen_range(0..total_weight);

    for &(item, weight) in options {
        if random_value < weight {
            return item; // Return the selected item
        }
        random_value -= weight; // Reduce the random_value by the current weight
    }

    // Fallback case, should never hit here if weights are set correctly
    options[0].0 // Return first option if all else fails
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng; // Standard RNG for testing
    use rand::SeedableRng;

    #[test]
    fn test_weighted_choose_uniform_distribution() {
        let options = [('A', 1), ('B', 1), ('C', 1)];
        let mut rng = StdRng::seed_from_u64(0); // Seed for reproducibility

        let results = (0..1000).map(|_| weighted_choose(&options, &mut rng)).collect::<Vec<_>>();

        let a_count = results.iter().filter(|&&x| x == 'A').count();
        let b_count = results.iter().filter(|&&x| x == 'B').count();
        let c_count = results.iter().filter(|&&x| x == 'C').count();

        // Since all options are equal, we expect counts to be roughly equal
        assert!((a_count as f64 - b_count as f64).abs() < 100.0);
        assert!((b_count as f64 - c_count as f64).abs() < 100.0);
    }

    #[test]
    fn test_weighted_choose_non_uniform_distribution() {
        let options = [('A', 1), ('B', 2), ('C', 3)];
        let mut rng = StdRng::seed_from_u64(1); // Different seed for reproducibility

        let results = (0..6000).map(|_| weighted_choose(&options, &mut rng)).collect::<Vec<_>>();

        let a_count = results.iter().filter(|&&x| x == 'A').count();
        let b_count = results.iter().filter(|&&x| x == 'B').count();
        let c_count = results.iter().filter(|&&x| x == 'C').count();

        // Expect c to be higher than b to be higher than a
        assert!(c_count > b_count);
        assert!(b_count > a_count);

        // Expect c roughly 3000, b roughly 2000, and a roughly 1000
        assert!((c_count as f64 - 3000 as f64).abs() < 100.0);
        assert!((b_count as f64 - 2000 as f64).abs() < 100.0);
        assert!((a_count as f64 - 1000 as f64).abs() < 100.0);

    }

    #[test]
    fn test_weighted_choose_empty_options() {
        let options: [(char, u32); 0] = [];

        // The function should panic if it is called with an empty array
        let result = std::panic::catch_unwind(|| {
            let mut rng = StdRng::seed_from_u64(2);
            weighted_choose(&options, &mut rng)
        });
        
        assert!(result.is_err()); // Ensure that it panics
    }

    #[test]
    fn test_weighted_choose_single_option() {
        let options = [('A', 10)];
        let mut rng = StdRng::seed_from_u64(0);

        let result = weighted_choose(&options, &mut rng);
        assert_eq!(result, 'A'); // The only option should always be chosen
    }

}

