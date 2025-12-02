fn parse_rotation_type(rotation: &str) -> i32 {
    let (direction, num_str): (&str, &str) = rotation.split_at(1);
    
    let num: i32 = num_str.parse().expect("Failed to parse number");
    match direction {
        "L" => -num,
        "R" => num,
        _ => panic!("Invalid rotation direction"),
    }
}
fn calculate_rotations(counter: &mut i32, zero_hits: &mut i32) {
    const MIN: i32 = 0;
    const MAX: i32 = 100;
    *counter = ((*counter % MAX) + MAX) % MAX;

    if *counter == MIN {
        *zero_hits += 1;
    }
}

fn main() {
    let mut counter: i32 = 50; // starts at 50
    let mut zero_hits: i32 = 0; // number of times we end on zero

    // Read and process each line from input.txt
    let contents: String = std::fs::read_to_string("src/input.txt")
        .expect("Failed to read input.txt");

    for line in contents.lines() {
        let line: &str = line.trim();
        if line.is_empty() {
            continue;
        } else {
            counter += parse_rotation_type(line);
            calculate_rotations(&mut counter, &mut zero_hits);
        }
    }

    eprintln!("zero_hits: {}", zero_hits);
}
