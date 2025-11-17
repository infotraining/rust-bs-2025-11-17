const GLOBAL_NUMBER: u32 = 42;
const GLOBAL_TEXT: &str = "Global Text";

static MESSAGE: &str = "Rust 2024";

static mut COUNTER: u32 = 0;

fn main() {
    unsafe {
        COUNTER += 1;
    }
    
    let local_counter;
    unsafe {
        local_counter = COUNTER;
    }

    println!("{MESSAGE} - {GLOBAL_NUMBER} - {local_counter}");
    
    let text= "Hello, world!";

    println!("Text: {} - length: {}", text, text.len());

    let mut number = 42; // int number = 42;
    number += 1;
    number = std::i32::MAX;
    println!("number: {number}");
    
    let result = number.checked_add(1);
    println!("result: {result:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_length() {
        let text = "Hello, world!";
        assert_eq!(text.len(), 13);
    }
}