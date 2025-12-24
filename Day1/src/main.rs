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

    fn rotate(&mut self, rotation: &str) -> i32 {
        let mut chars = rotation.chars();
        let direction = match chars.next() {
            Some('L') => "L",
            Some('R') => "R",
            _ => panic!("must start with L or R"),
        };

        let number_str: String = chars.collect();
        let number = number_str.parse::<i32>().unwrap();

        if direction == "R" {
            self.current_position = (self.current_position + number) % self.max_position;
        } else {
            let new_position: i32 =
                self.current_position as i32 - (number % self.max_position) as i32;

            self.current_position = match new_position {
                n if n < 0 => (self.max_position as i32 + new_position) as i32,
                _ => new_position as i32,
            }
        }

        self.current_position
    }
}

fn calculate_secret_code(rotations: Vec<String>) -> i32 {
    let mut dial = Dial::default();
    let mut dial_at_zero_count = 0 as i32;
    for (i, rotation) in rotations.iter().enumerate() {
        let new_position = dial.rotate(rotation);
        println!("The dial is rotated {rotation} to point at {new_position}.[Index: {i}]");
        if new_position == 0 {
            dial_at_zero_count += 1;
        }
    }
    dial_at_zero_count
}

// This function is AI generated:
fn read_rotations_file<P: AsRef<Path>>(file_path: P) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let rotations: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    Ok(rotations)
}

fn main() {
    let rotations = read_rotations_file("./src/rotations.txt").unwrap();
    let len: usize = rotations.len();
    println!("Read {len} lines");

    let secret_code = calculate_secret_code(rotations);
    print!("The secret code is {secret_code}");
}

#[test]
fn test_rotate() {
    let mut dial = Dial::default();
    assert_eq!(dial.rotate("R5"), 55);
    assert_eq!(dial.rotate("R5"), 60);
    assert_eq!(dial.rotate("L60"), 0);
    assert_eq!(dial.rotate("R5"), 5);
    assert_eq!(dial.rotate("L10"), 95);
    assert_eq!(dial.rotate("L99"), 96);

    let mut dial = Dial::default();
    assert_eq!(dial.rotate("L50"), 0);
    assert_eq!(dial.rotate("L9"), 91);
    assert_eq!(dial.rotate("L2"), 89);
    assert_eq!(dial.rotate("R42"), 31);
    assert_eq!(dial.rotate("R41"), 72);
    assert_eq!(dial.rotate("R5"), 77);
    assert_eq!(dial.rotate("R23"), 0);
    let mut dial = Dial {
        current_position: 0,
        max_position: 100,
    };
    assert_eq!(dial.rotate("R84"), 84);
    assert_eq!(dial.rotate("L484"), 0);
}

#[test]
fn test_part_1() {
    let rotations = [
        "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    let secret_code = calculate_secret_code(rotations);
    assert_eq!(secret_code, 3);

    let rotations = ["L50", "L9", "L2", "R42", "R41", "R5", "R23"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let secret_code = calculate_secret_code(rotations);
    assert_eq!(secret_code, 2);
}
