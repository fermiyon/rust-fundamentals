/// This module provides utilities for handling file sizes, allowing users to input a string
/// representing the size and unit (e.g., "300 kb" or "12 mb"). It defines a `Sizes` struct
/// that holds the file size in various units (bytes, kilobytes, megabytes, gigabytes) and
/// returns a debug representation of this struct. The module includes an enum `FileSize`
/// with variants for different size units and functions to parse the input string and
/// format the file sizes accordingly.

#[derive(Debug)]
enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}

impl FileSize {
    /// Parses an input string (e.g., "300 kb" or "12 mb") and returns the corresponding `FileSize`.
    fn from_input(input: &str) -> Option<FileSize> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() != 2 {
            return None; // Invalid input format
        }

        let size: u64 = parts[0].parse().ok()?;
        let unit = parts[1].to_lowercase();

        match unit.as_str() {
            "bytes" => Some(FileSize::Bytes(size as u64)),
            "kb" => Some(FileSize::Kilobytes(size as f64)),
            "mb" => Some(FileSize::Megabytes(size as f64)),
            "gb" => Some(FileSize::Gigabytes(size as f64)),
            _ => None, // Unknown unit
        }
    }
}

#[derive(Debug)]
struct Sizes {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
}

impl Sizes {
    /// Creates a new `Sizes` instance from a `FileSize`.
    fn from_file_size(file_size: FileSize) -> Sizes {
        match file_size {
            FileSize::Bytes(b) => Sizes {
                bytes: format!("{} bytes", b),
                kilobytes: format!("{} kilobytes", b as f64 / 1000.0),
                megabytes: format!("{} megabytes", b as f64 / 1_000_000.0),
                gigabytes: format!("{} gigabytes", b as f64 / 1_000_000_000.0),
            },
            FileSize::Kilobytes(kb) => Sizes {
                bytes: format!("{} bytes", kb as u64 * 1000),
                kilobytes: format!("{} kilobytes", kb),
                megabytes: format!("{} megabytes", kb / 1000.0),
                gigabytes: format!("{} gigabytes", kb / 1_000_000.0),
            },
            FileSize::Megabytes(mb) => Sizes {
                bytes: format!("{} bytes", mb as u64 * 1_000_000),
                kilobytes: format!("{} kilobytes", mb * 1000.0),
                megabytes: format!("{} megabytes", mb),
                gigabytes: format!("{} gigabytes", mb / 1000.0),
            },
            FileSize::Gigabytes(gb) => Sizes {
                bytes: format!("{} bytes", gb as u64 * 1_000_000_000),
                kilobytes: format!("{} kilobytes", gb * 1_000_000.0),
                megabytes: format!("{} megabytes", gb * 1000.0),
                gigabytes: format!("{} gigabytes", gb),
            },
        }
    }
}

fn main() {
    // Example usage:
    if let Some(input) = std::env::args().nth(1) {
        if let Some(file_size) = FileSize::from_input(&input) {
            let sizes = Sizes::from_file_size(file_size);
            println!("Sizes: {:?}", sizes);
        } else {
            println!("Invalid input format. Usage: cargo run -- <size>");
        }
    } else {
        println!("No file size provided. Usage: cargo run -- <size>");
    }
}
