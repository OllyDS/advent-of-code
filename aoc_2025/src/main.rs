fn parse_rotation_type(rotation: &str) -> i32 {
    let (direction, num_str): (&str, &str) = rotation.split_at(1);
    let num: i32 = num_str.parse().expect("Failed to parse number");

    match direction {
        "L" => -num,
        "R" => num,
        _ => panic!("Invalid rotation direction"),
    }
}

fn calculate_rotations(counter: &mut i32, rotation: i32, zero_hits: &mut i32) {
    const MIN: i32 = 0;
    const MAX: i32 = 100;

    // Prevents double counting when going from 0 to negative.
    // For example, 0 - 1 == 99 - this will trigger the below while loop
    // despite us not passing through 0 because we started there.
    // By subtracting one from zero_hits here, we negate that extra count in the while loop.
    if *counter == MIN && (*counter + rotation < MIN) {
        *zero_hits -= 1;
    }

    *counter += rotation;
    while *counter < MIN || *counter > MAX {
        if *counter > MAX {
            *counter -= MAX;
        } else if *counter < MIN {
            *counter += MAX;
        }
        *zero_hits += 1;
    }

    if *counter == MAX {
        *counter = MIN; // Reset to 0 if it's exactly 100
    }

    if *counter == MIN {
        *zero_hits += 1;
    }
}

fn main() {
    let mut counter: i32 = 50; // starts at 50
    let mut zero_hits: i32 = 0;

    // Read and process each line from input.txt
    let contents: String = std::fs::read_to_string("src/input.txt")
        .expect("Failed to read input.txt");

    for line in contents.lines() {
        let line: &str = line.trim();
        if line.is_empty() {
            continue;
        } else {
            let rotation: i32 = parse_rotation_type(line);
            calculate_rotations(&mut counter, rotation, &mut zero_hits);
        }
    }

    eprintln!("zero_hits: {}", zero_hits);
}
