use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Dial {
    current_position: i32,
    max_position: i32,
}

impl Default for Dial {
    fn default() -> Self {
        Dial {
            current_position: 50,
            max_position: 100,
        }
    }
}

impl Dial {
    // Simplifying the problem for part 2.
    fn clock() -> Self {
        Dial {
            current_position: 6,
            max_position: 12,
        }
    }

    fn rotate(&mut self, rotation: &str) -> (i32, i32) {
        let original_position = self.current_position;
        let mut chars = rotation.chars();
        let direction = chars.next().unwrap().to_string();

        let number_str: String = chars.collect();
        let number = number_str.parse::<i32>().unwrap();

        // For Part 1
        if direction == "R" {
            self.current_position = (self.current_position + number) % self.max_position;
        } else {
            let new_position = self.current_position - (number % self.max_position);

            self.current_position = match new_position {
                n if n < 0 => self.max_position + new_position,
                _ => new_position,
            }
        }

        // For Part 2
        let full_rotations_count = number / self.max_position;
        let overflow_count = match (
            direction.as_str(),
            original_position + (number % self.max_position) > self.max_position,
        ) {
            ("R", true) => 1i32,
            _ => 0i32,
        };
        let underflow_count = match (
            direction.as_str(),
            self.current_position + (number % self.max_position) > self.max_position,
        ) {
            ("L", true) => 1i32,
            _ => 0i32,
        };
        let at_zero = match self.current_position {
            0 => 1i32,
            _ => 0i32,
        };
        let passed_or_at_zero_count =
            full_rotations_count + overflow_count + underflow_count + at_zero;

        println!("Original Position: {original_position}");
        println!("Rotation: {rotation}");
        println!("New Position: {}", self.current_position);
        println!(
            "Full Rotations: {number}/{} = {full_rotations_count}",
            self.max_position
        );
        println!("Overflow Count: {overflow_count}");
        println!("Underflow Count: {underflow_count}");
        println!("Passed or at Zero Count: {passed_or_at_zero_count} ");
        println!("--------------------");
        println!("");

        (self.current_position, passed_or_at_zero_count)
    }
}

fn calculate_secret_code(dial: &mut Dial, rotations: Vec<String>) -> (i32, i32) {
    let mut total_dial_at_zero_count = 0i32;
    let mut total_dial_passed_or_at_zero_count = 0i32;

    for rotation in rotations.iter() {
        let (new_position, passed_or_at_zero_count) = dial.rotate(rotation);
        log_rotation(rotation, new_position, passed_or_at_zero_count);

        total_dial_passed_or_at_zero_count += passed_or_at_zero_count;
        if new_position == 0 {
            total_dial_at_zero_count += 1;
        }
    }

    (total_dial_at_zero_count, total_dial_passed_or_at_zero_count)
}

fn log_rotation(rotation: &str, new_position: i32, passed_or_at_zero_count: i32) {
    let mut log = format!("- The dial is rotated {rotation} to point at {new_position}");
    if passed_or_at_zero_count > 0 {
        log.push_str(
            format!("; during this rotation, it points at 0 {passed_or_at_zero_count} times")
                .as_str(),
        );
    }
    log.push_str(".");
    println!("{}", log);
}

// This function is AI generated:
fn read_file<P: AsRef<Path>>(file_path: P) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let rotations: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    Ok(rotations)
}

fn main() {
    let rotations = read_file("./src/rotations.txt").unwrap();
    let len: usize = rotations.len();
    println!("Read {len} lines");

    let mut dial = Dial::default();
    let (part_one, part_two) = calculate_secret_code(&mut dial, rotations);
    println!("The secret code is\n\t- Part 1:{part_one}\n\t- Part 2: {part_two}");
}

#[test]
fn test_rotate_for_part_1() {
    let mut dial = Dial::default();
    assert_eq!(dial.rotate("R5").0, 55);
    assert_eq!(dial.rotate("R5").0, 60);
    assert_eq!(dial.rotate("L60").0, 0);
    assert_eq!(dial.rotate("R5").0, 5);
    assert_eq!(dial.rotate("L10").0, 95);
    assert_eq!(dial.rotate("L99").0, 96);

    let mut dial = Dial::default();
    assert_eq!(dial.rotate("L50").0, 0);
    assert_eq!(dial.rotate("L9").0, 91);
    assert_eq!(dial.rotate("L2").0, 89);
    assert_eq!(dial.rotate("R42").0, 31);
    assert_eq!(dial.rotate("R41").0, 72);
    assert_eq!(dial.rotate("R5").0, 77);
    assert_eq!(dial.rotate("R23").0, 0);
    let mut dial = Dial {
        current_position: 0,
        max_position: 100,
    };
    assert_eq!(dial.rotate("R84").0, 84);
    assert_eq!(dial.rotate("L484").0, 0);
}

#[test]
fn test_secret_code_for_part_1() {
    let mut dial = Dial::default();
    let rotations = [
        "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    let (part_one, _) = calculate_secret_code(&mut dial, rotations);
    assert_eq!(part_one, 3);

    let rotations = ["L50", "L9", "L2", "R42", "R41", "R5", "R23"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let (part_one, _) = calculate_secret_code(&mut dial, rotations);
    assert_eq!(part_one, 2);
}

#[test]
fn test_rotate_for_part_2() {
    let mut dial = Dial::clock();
    assert_eq!(dial.rotate("R1"), (7, 0));
    assert_eq!(dial.rotate("R3"), (10, 0));
    assert_eq!(dial.rotate("R3"), (1, 1));
    assert_eq!(dial.rotate("R6"), (7, 0));
    assert_eq!(dial.rotate("R9"), (4, 1));
    assert_eq!(dial.rotate("R27"), (7, 2));
    assert_eq!(dial.rotate("L3"), (4, 0));
    assert_eq!(dial.rotate("R32"), (0, 3));
    assert_eq!(dial.rotate("R7"), (7, 0));
    assert_eq!(dial.rotate("L1"), (6, 0));
    assert_eq!(dial.rotate("L2"), (4, 0));
    assert_eq!(dial.rotate("L4"), (0, 1));
    assert_eq!(dial.rotate("L8"), (4, 0));
    assert_eq!(dial.rotate("L36"), (4, 3));
    assert_eq!(dial.rotate("L40"), (0, 4));
}

#[test]
fn test_secret_code_for_part_2() {
    let mut dial = Dial::clock();
    let rotations = [
        "R1", "R3", "R3", "R6", "R9", "R27", "L3", "R32", "R7", "L1", "L2", "L4", "L8", "L36",
        "L40",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    let (part_one, part_two) = calculate_secret_code(&mut dial, rotations);
    assert_eq!(part_one, 3);
    assert_eq!(part_two, 15);
}
