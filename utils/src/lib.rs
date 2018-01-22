
pub fn word_count(text: &str) -> u32 {
    let mut word_counter = 0;
    for _word in text.split_whitespace() {
        word_counter += 1; 
    }
    word_counter
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
