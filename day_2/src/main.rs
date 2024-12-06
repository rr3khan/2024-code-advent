// Red-Nosed reactor safety systems tolerate levels problem

use day_2::{is_lvl_diff_within_1_to_3, is_only_increasing_or_decreasing};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Read the data from the input file
    let input_file = File::open("input.txt").expect("Unable to open file");
    let reader = BufReader::new(input_file);

    let mut num_safe_reports = 0;

    for line in reader.lines() {
        let line = line.expect("The input file should contain some data");

        // println!("report row {:?}", line);

        let mut line_data: Result<Vec<i32>, _> =
            line.split_whitespace().map(|s| s.parse::<i32>()).collect();

        match line_data {
            Ok(mut vec) => {
                if is_lvl_diff_within_1_to_3(vec.clone())
                    && is_only_increasing_or_decreasing(&mut vec)
                {
                    num_safe_reports = num_safe_reports + 1;
                    // println!("match true");
                }
                // println!("report row vec {:?}", vec);
            }
            Err(_) => {
                println!("Error: Data should only contain numbers");
                // skip errored lines
            }
        }
    }

    // star_map_distances(&mut list_1, &mut list_2);

    // similarity_score(&mut list_1, &mut list_2);
    println!("Total number of safe reports {:?}", num_safe_reports);
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_is_lvl_diff_within_1_to_3() {
        let mut reports = vec![1, 3, 6, 7, 9];
        assert!(is_lvl_diff_within_1_to_3(reports));
    }

    #[test]
    fn test_is_only_increasing_or_decreasing() {
        let mut reports = vec![7, 6, 4, 2, 1];
        assert!(is_only_increasing_or_decreasing(&mut reports));
    }
}
