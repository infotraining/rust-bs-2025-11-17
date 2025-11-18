use std::i32;

const GLOBAL_NUMBER: u32 = 2025;

static MESSAGE: &str = "Rust";
static mut COUNTER: u32 = 0;

fn overflow_integer() {
    let mut number = i32::MAX;
    // number += 1; // cause overflow
    number = number.wrapping_add(1);
    println!("number after overflow: {number}");
}

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

    overflow_integer();
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