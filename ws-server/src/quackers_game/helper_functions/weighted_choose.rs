use rand::{thread_rng, Rng};

pub fn weighted_choose<T: Copy>(options: &[(T, u32)]) -> T {
    let mut rng = thread_rng();
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
