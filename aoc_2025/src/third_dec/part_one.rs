pub mod input_processor;

fn largest_jolt_pair(bank: Vec<i128>) -> (i128, i128) {
    // First step is to find the largest number in the entire vector
    let mut left_num: i128 = 0;
    let mut left_num_idx: usize = 0;
    // Use split_last to remove the last element as that's reserved for the left num starting point
    for (idx, &num) in bank.split_last().unwrap().1.iter().enumerate() {
        if num > left_num {
            left_num = num;
            left_num_idx = idx;
        }
    }

    // Next we use the largest number to split the vector into two halves
    let mut right_num: i128 = 0;
    let (_, remaining_bank) = bank.split_at(left_num_idx + 1);

    println!("remaining_bank: {:?}", remaining_bank);

    for &num in remaining_bank {
        if num > right_num {
            right_num = num;
        }
    }

    (left_num, right_num)
}

fn main() {
    let vec_str: Vec<String> =
        input_processor::InputProcessor::new("src/input.txt").read_lines();

    let parsed_bank: Vec<Vec<i128>> = vec_str
        .iter()
        .map(|bank| { // Split each 'bank' string into individual characters and parse to i128
            let bank_vec: Vec<char> = bank.chars().collect();
            println!("bank_vec: {:?}", bank_vec);
            return bank_vec.iter().map(|b| b.to_string().parse::<i128>().expect(&format!("Failed to parse bank {:?}", bank_vec))).collect();
        })
        .collect::<Vec<Vec<i128>>>();

    let jolts: Vec<i128> = parsed_bank
        .iter()
        .map(|bank| {
            // Extract the two jolt figures
            let (left_jolt, right_jolt) = largest_jolt_pair(bank.to_vec());
            // Combine them into a single number using string formatting - (9, 8) --> "98"
            let combined_jolts: String = format!("{left_jolt}{right_jolt}");
            // Parse back to i128 and return
            return combined_jolts.parse::<i128>().expect("Failed to parse combined jolts");
        })
        .collect();

    eprintln!("jolts: {:?}", jolts.iter().sum::<i128>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_jolt_pair() {
        assert_eq!(largest_jolt_pair([1,2,3,4,5].to_vec()), (4, 5));
        assert_eq!(largest_jolt_pair([7, 7].to_vec()), (7, 7));
        assert_eq!(largest_jolt_pair([3, 4].to_vec()), (3, 4));
        assert_eq!(largest_jolt_pair([9,8,9,8,9].to_vec()), (9, 9));
    }
}
