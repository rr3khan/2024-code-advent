use day_1::{similarity_score, star_map_distances};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the data from the input file
    let input_file = File::open("input.txt").expect("Unable to open file");
    let reader = BufReader::new(input_file);

    let mut list_1 = Vec::new();
    let mut list_2 = Vec::new();

    for line in reader.lines() {
        let line = line?;

        let mut parts = line.split_whitespace();

        if let (Some(part1), Some(part2)) = (parts.next(), parts.next()) {
            let num1: i32 = part1.parse()?;
            let num2: i32 = part2.parse()?;

            list_1.push(num1);
            list_2.push(num2);
        }
    }

    star_map_distances(&mut list_1, &mut list_2);

    similarity_score(&mut list_1, &mut list_2);

    Ok(())
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_star_map_distances() {
        let mut list_1 = vec![3, 4, 2, 1, 3, 3];
        let mut list_2 = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(star_map_distances(&mut list_1, &mut list_2), 11);
    }

    #[test]
    fn test_similarity_scores() {
        let mut list_1 = vec![3, 4, 2, 1, 3, 3];
        let mut list_2 = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(similarity_score(&mut list_1, &mut list_2), 31);
    }
}
