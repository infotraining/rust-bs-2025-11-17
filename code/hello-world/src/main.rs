fn main() {
    let text= "Hello, world!";

    println!("Text: {} - length: {}", text, text.len());
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