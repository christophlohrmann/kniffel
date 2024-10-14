use std::collections::HashMap;

pub fn vecs_elementwise_equal(vec1: &Vec<i32>, vec2: &Vec<i32>) -> bool {
    if vec1.len() != vec2.len() {
        return false;
    }
    return vec1.iter().zip(vec2).filter(|&(a, b)| a == b).count() == vec1.len();
}

pub fn count_unique_elements(vec: &Vec<i32>) -> HashMap<i32, usize> {
    let mut counts = HashMap::new();
    for &value in vec.iter() {
        counts
            .entry(value)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    return counts;
}

pub fn set_slice_from_vec(dest: &mut Vec<i32>, src: Vec<i32>, start_index: usize) {
    // Check if the starting index is within bounds
    if start_index + src.len() > dest.len() {
        panic!("Source vector is too large for destination vector at the given index.");
    }

    // Copy the elements from src to the appropriate slice in dest
    dest[start_index..start_index + src.len()].copy_from_slice(&src);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_elementwise_eq() {
        let a = vec![1, 1, 2, 3];
        let same = vec![1, 1, 2, 3];
        let too_long = vec![1, 1, 2, 3, 4];
        let diff_elements = vec![1, 1, 2, 4];
        let diff_order = vec![1, 2, 1, 3];

        assert!(vecs_elementwise_equal(&a, &same));
        assert!(!vecs_elementwise_equal(&a, &too_long));
        assert!(!vecs_elementwise_equal(&a, &diff_elements));
        assert!(!vecs_elementwise_equal(&a, &diff_order));
    }

    #[test]
    fn test_count_unique_elements() {
        let a = vec![1, 1, 2, 1, 3, 3, 4];
        let uniqe_counts = count_unique_elements(&a);
        let should_be: HashMap<i32, usize> =
            [(1, 3), (2, 1), (3, 2), (4, 1)].iter().cloned().collect();
        assert_eq!(uniqe_counts, should_be);
    }
}
