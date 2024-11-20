// strings1.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings1` for hints ;)

fn main() {
    let answer = current_favorite_course();
    println!("My course is {}", answer);
}

fn current_favorite_course() -> String {
    String::from("Solana")
    // String::from is a method that creates a new String from a string literal
    // Also we can use "Solana".to_string() 
}
