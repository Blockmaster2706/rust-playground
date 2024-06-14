use rand::Rng;

pub fn get_random_word() -> String {
    let words = vec!["hello", "world", "rust", "programming"];
    let random_index = rand::thread_rng().gen_range(0..words.len());
    words[random_index].to_string()
}

pub fn is_word_guessed(word: &String, guessed: &Vec<String>) -> bool {
    let mut guessed_word = String::new();

    for c in word.chars() {
        if guessed.contains(&c.to_string()) {
            guessed_word.push(c);
        } else {
            guessed_word.push('_');
        }
    }

    guessed_word == *word
}

pub fn hide_word(word: &String, guessed: &Vec<String>) -> String {
    let mut guessed_word = String::new();

    for c in word.chars() {
        if guessed.contains(&c.to_string()) {
            guessed_word.push(c);
        } else {
            guessed_word.push('_');
        }
    }

    guessed_word
}