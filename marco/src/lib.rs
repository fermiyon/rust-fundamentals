/* A Marco Polo Game

If the name Marco is given, the program will responds woth Polo
Otherwise, the program will respionds with "What's your name?"
 */

pub fn marco_polo(name: &str) -> String {
    if name == "Marco" {
        "Polo".to_string()
    } else {
        "What's your name?".to_string()
    }
}
