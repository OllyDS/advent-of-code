/// Given a range in the format "X-Y",
/// create a vector of integers from X to Y inclusive.
/// #### Example
/// "3-5" --> [3, 4, 5]
fn create_vec_range(range: &str) -> Vec<i64> {
    let (first, last) = range
        .split_once('-')
        .expect("Invalid range format, expected 'X-Y'");
    let first_num: i64 = first.trim().parse().expect(&format!("Failed to parse first number: {}", first));
    let last_num: i64 = last.trim().parse().expect(&format!("Failed to parse last number: {}", last));

    return (first_num..=last_num).collect();
}

/// Check if a number is an invalid ID (sequence repeated at least twice)
fn is_invalid_id(num: i64) -> bool {
    let num_str: String = num.to_string();
    
    if num_str.len() < 2 {
        return false; // Must have at least 2 digits
    }
    
    // Creates an inclusive range from 1 to half the string length
    // For a pattern to repeat at least twice, it can be at most half the total length:
    // "1111" (length 4) --> max pattern length is 2 ("11" repeated twice)
    // "123123" (length 6) --> max pattern length is 3 ("123" repeated twice)
    // "12341234" (length 8) --> max pattern length is 4 ("1234" repeated twice)
    let pattern_range: Vec<usize> = (1..=(num_str.len() / 2)).collect();

    // Try each possible pattern length
    for range_num in pattern_range {
        let pattern: &str = &num_str[0..range_num];
        
        // Skip patterns starting with 0
        if pattern.starts_with('0') {
            continue;
        }
        
        // Check if repeating this pattern creates the entire string
        let repeated: String = pattern.repeat(num_str.len() / range_num);
        if repeated == num_str && num_str.len() / range_num >= 2 {
            return true;
        }
    }
    
    false
}

fn main() {
    // Read and process each line from input.txt
    let contents: String = std::fs::read_to_string("src/input.txt")
        .expect("Failed to read input.txt");

    let vec_collection: Vec<Vec<i64>> = contents
        .split(',')
        .map(|range: &str| create_vec_range(range.trim()))
        .collect::<Vec<Vec<i64>>>();

    let sum_invalid: i64 = vec_collection.iter().map(|range| {
        range.iter()
            .filter(|&&num| is_invalid_id(num))
            .sum::<i64>()
    }).sum();
    eprintln!("Sum of invalid IDs: {}", sum_invalid);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_invalid_id_valid_cases() {
        // Examples from the problem description
        assert!(is_invalid_id(55));      // 5 twice
        assert!(is_invalid_id(6464));    // 64 twice  
        assert!(is_invalid_id(123123));  // 123 twice
        assert!(is_invalid_id(11));      // 1 twice
        assert!(is_invalid_id(38593859)); // 3859 twice
    }

    #[test]
    fn test_is_invalid_id_invalid_cases() {
        // Single digits
        assert!(!is_invalid_id(5));
        assert!(!is_invalid_id(0));

        // Odd length numbers
        assert!(!is_invalid_id(123));
        assert!(!is_invalid_id(12345));

        // Numbers that don't repeat
        assert!(!is_invalid_id(12));
        assert!(!is_invalid_id(1234));
        assert!(!is_invalid_id(101));

        // Would have leading zeros if split
        assert!(!is_invalid_id(1001)); // Would be "10" + "01"
        assert!(!is_invalid_id(2002)); // Would be "20" + "02"
    }

    #[test]
    fn test_create_vec_range() {
        assert_eq!(create_vec_range("3-5"), vec![3, 4, 5]);
        assert_eq!(create_vec_range("10-12"), vec![10, 11, 12]);
        assert_eq!(create_vec_range("55-55"), vec![55]);
        assert_eq!(create_vec_range("11-22"), vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22]);
    }
}
