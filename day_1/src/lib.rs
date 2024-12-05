/// Calculates the "distance" between two lists of integers.
///
/// The function sorts both lists, aligns them by index, and computes the
/// absolute difference between corresponding elements. These differences
/// are summed to produce the final "star map distance."
///
/// # Arguments
///
/// * `list_1`: i32 - First list of locations.
/// * `list_2`: i32 - Second list of locations.
///
/// # Returns
///
/// An `i32` representing the total "star map distance."
///
pub fn star_map_distances(list_1: &mut Vec<i32>, list_2: &mut Vec<i32>) -> i32 {
    // Sort both lists in numerical order to get smallest
    // nums of both aligned w.r.t. index
    list_1.sort();
    list_2.sort();

    let mut star_distance = 0;

    for (index, num) in list_1.iter().enumerate() {
        star_distance = star_distance + (num - list_2[index]).abs();
    }

    // println!("list 1 {:?}", list_1);
    // println!("list 2 {:?}", list_2);
    println!("Star Dist: {:?}", star_distance);
    return star_distance;
}

/// Calculates the "similarity score" between two star lists
///
/// The function uses the given definition of a similarity between
/// two locations as being the number of time location 1 appears in list 2
/// multiplied by the value of location 1
/// similarities for all locations in list 1 are summed to produce the final "similarity score"
///
/// # Arguments
///
/// * `list_1`: i32 - First list of locations.
/// * `list_2`: i32 - Second list of locations.
///
/// # Returns
///
/// An `i32` representing the total "similarity score"
///
pub fn similarity_score(list_1: &mut Vec<i32>, list_2: &mut Vec<i32>) -> i32 {
    list_1.sort();
    list_2.sort();

    let mut similarity = 0;

    for (index, num) in list_1.iter().enumerate() {
        similarity = similarity + *num * list_2.iter().filter(|num2| *num2 == num).count() as i32;
    }

    println!("Similarity Score: {:?}", similarity);

    return similarity;
}
