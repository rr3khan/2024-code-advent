pub fn is_only_increasing_or_decreasing(reports_row: &mut Vec<i32>) -> bool {
    let mut trend = None;

    for window in reports_row.windows(2) {
        let diff = window[1] - window[0];
        if diff == 0 {
            return false; //  Not strictly increasing or decreasing
        }

        let current_trend = if diff > 0 { 1 } else { -1 };

        match trend {
            None => trend = Some(current_trend),           // Initial Trend
            Some(t) if t != current_trend => return false, // Trend switch
            _ => {}                                        // Trend continues
        }
    }

    return true;
}

pub fn is_lvl_diff_within_1_to_3(reports_row: Vec<i32>) -> bool {
    for (index, num) in reports_row.iter().enumerate() {
        if index + 1 == reports_row.len() {
            break;
        }

        let mut diff = (num - reports_row[index + 1]).abs();

        if diff < 1 || diff > 3 {
            return false;
        }
    }
    return true;
}
