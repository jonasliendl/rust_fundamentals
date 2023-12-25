
fn find_longest_word(str: String) -> String {
    let mut longest_word = String::new();
    for word in str.split_whitespace() {
        if word.len() > longest_word.len() {
            longest_word = word.to_string();
        }
    }
    longest_word
}

fn main() {
    let sentence = "the quick brown fox jumps over the lazy dog".to_string();
    // Use slicing to get the first three characters of the sentence
    //println!("{}", &sentence[0..=4]);

    // concatenate using format!
    let description = format!("Title: Quick story\n{}", sentence);
    //println!("{}", description);

    let longest_word = find_longest_word(description);
    println!("Longest word: {} with length {}", longest_word, longest_word.len());

    // iterate over the characters in the sentence
    let mut count = 0;
    for c in sentence.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => count+=1,
            _ => continue,
        }
    }
    println!("Number of vowels: {}", count);

    // Split and collect into a vector
    //let words: Vec<&str> = sentence.split_whitespace().collect();
    let words = sentence.split(' ').collect::<Vec<&str>>();
    //println!("{:?}", words);

    let reversed = sentence.chars().rev().collect::<String>();
    println!("{}", reversed);
}
