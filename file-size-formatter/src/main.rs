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
            eprintln!("Invalid input format. Usage: cargo run -- <size>");
            return None;
        }

        let size: f64 = match parts[0].parse() {
            Ok(val) => val,
            Err(_) => {
                eprintln!("Invalid size value: {}", parts[0]);
                return None;
            }
        };

        if size < 0.0 {
            eprintln!("Size cannot be negative: {}", size);
            return None;
        }

        let unit = parts[1].to_lowercase();

        match unit.as_str() {
            "bytes" => Some(FileSize::Bytes(size as u64)),
            "kb" => Some(FileSize::Kilobytes(size)),
            "mb" => Some(FileSize::Megabytes(size)),
            "gb" => Some(FileSize::Gigabytes(size)),
            _ => {
                eprintln!("Unknown unit: {}", unit);
                None
            }
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
                kilobytes: format!("{:.2} KB", b as f64 / 1000.0),
                megabytes: format!("{:.2} MB", b as f64 / 1_000_000.0),
                gigabytes: format!("{:.2} GB", b as f64 / 1_000_000_000.0),
            },
            FileSize::Kilobytes(kb) => Sizes {
                bytes: format!("{} bytes", (kb * 1000.0) as u64),
                kilobytes: format!("{:.2} KB", kb),
                megabytes: format!("{:.2} MB", kb / 1000.0),
                gigabytes: format!("{:.2} GB", kb / 1_000_000.0),
            },
            FileSize::Megabytes(mb) => Sizes {
                bytes: format!("{} bytes", (mb * 1_000_000.0) as u64),
                kilobytes: format!("{:.2} KB", mb * 1000.0),
                megabytes: format!("{:.2} MB", mb),
                gigabytes: format!("{:.2} GB", mb / 1000.0),
            },
            FileSize::Gigabytes(gb) => Sizes {
                bytes: format!("{} bytes", (gb * 1_000_000_000.0) as u64),
                kilobytes: format!("{:.2} KB", gb * 1_000_000.0),
                megabytes: format!("{:.2} MB", gb * 1000.0),
                gigabytes: format!("{:.2} GB", gb),
            },
        }
    }
}

fn main() {
    // Example usage:
    if let Some(input) = std::env::args().nth(1) {
        if let Some(file_size) = FileSize::from_input(&input) {
            let sizes = Sizes::from_file_size(file_size);
            // Bytes
            println!("Bytes: {}", sizes.bytes);
            // Kilobytes
            println!("Kilobytes: {}", sizes.kilobytes);
            // Megabytes
            println!("Megabytes: {}", sizes.megabytes);
            // Gigabytes
            println!("Gigabytes: {}", sizes.gigabytes);
            println!("Sizes: {:?}", sizes);
        } else {
            eprintln!("Invalid input format. Usage: cargo run -- <size>");
        }
    } else {
        eprintln!("No file size provided. Usage: cargo run -- <size>");
    }
}
