fn main() {
    println!("Hello, world!");
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_star_map_distances() {
        let list_1 = vec![3, 4, 2, 1, 3, 3];
        let list_2 = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(star_map_distances(), 11);
    }
}
