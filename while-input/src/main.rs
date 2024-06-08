use std::io;

// This example is a useful application of while because it allows to continue
// Asking for user input until the user types a specific keyword in this case "stop"
fn main() {
    let mut input = String::new();

    while input.trim() != "stop" {
        println!("Please type something: ");
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        println!("You typed: {}", input);
    }
    println!("Stopping...");
}
