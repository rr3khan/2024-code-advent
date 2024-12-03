fn main() {
    println!("Hello, world!");
}

#[cfg(test)]

/// Calculates the "distance" between two lists of integers.
///
/// The function sorts both lists, aligns them by index, and computes the
/// absolute difference between corresponding elements. These differences
/// are summed to produce the final "star map distance."
///
/// # Arguments
///
/// * `list_1`: i8 - First list of locations.
/// * `list_2`: i8 - Second list of locations.
///
/// # Returns
///
/// An `i8` representing the total "star map distance."
///
fn star_map_distances(list_1: &mut Vec<i8>, list_2: &mut Vec<i8>) -> i8 {
    // Sort both lists in numerical order to get smallest
    // nums of both aligned w.r.t. index
    list_1.sort();
    list_2.sort();

    let mut star_distance = 0;

    for (index, num) in list_1.iter().enumerate() {
        star_distance = star_distance + (num - list_2[index]).abs();
    }

    println!("list 1 {:?}", list_1);
    println!("list 2 {:?}", list_2);
    println!("Star Dist: {:?}", star_distance);
    return star_distance;
}

mod tests {
    use super::*;

    #[test]
    fn test_star_map_distances() {
        let mut list_1 = vec![3, 4, 2, 1, 3, 3];
        let mut list_2 = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(star_map_distances(&mut list_1, &mut list_2), 11);
    }
}
