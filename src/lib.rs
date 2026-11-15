//! This is the backend library for [`dwgen`](https://github.com/pgoodall/dwgen),
//! but can be used on it's own to build other products.
//! 
//! This early version of the library is very simple.

use std::io::Error;

pub struct Config {
    pub file: String,
    pub num: Option<u8>,
}

/// Pass your args to the `generate_password` function. 
/// Returns a `Vec<String>`
/// # Example:
/// ```
/// use dwgen_lib::{Config, generate_password};
/// let args = Config {
///             file: "test/example.txt".to_string(),
///             num: Some(5), };
/// let password = generate_password(args);
/// println!("Password: {:?}", password);
/// ```
pub fn generate_password(args: Config) -> Vec<String> {
    use rand::rngs;
    use rand::seq::{IteratorRandom, SliceRandom};

    // Ingest contens of the file and save the words in a Vec
    let mut words: Vec<String> = Vec::new();
    if let Ok(content) = process_file(&args.file) {
        content
            .split_whitespace()
            .for_each(|s| words.push(s.to_string()));
    };

    // Clean-up the word list
    let mut word_list: Vec<String> = clean_word_list(words);

    // Shuffle the words in the words list
    let mut rng = rngs::ThreadRng::default();
    word_list.shuffle(&mut rng);

    // Choose the specified number of random words from the list 
    // and print them out
    let mut password: Vec<String> = Vec::new();
    for _ in 0..= args.num.unwrap_or(5) {
        if let Some(word) = word_list.iter().choose(&mut rng) {
            password.push(word.to_string().to_lowercase());
        }
    };

    password
}

fn process_file(f: &String) -> Result<String, Error> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::open(f)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn clean_word_list(words: Vec<String>) -> Vec<String> {
    words
        .into_iter()
        .filter(|w| w.len() > 3)
        .map(|w| {
            w.replace(
                &['(', ')', ',', '\"', '.', ';', ':', '\'', '!', '-'][..],
                "",
            )
        })
        .collect()
}
