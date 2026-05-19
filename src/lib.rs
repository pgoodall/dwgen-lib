//! This is the backend library for [`dwgen`](https://github.com/pgoodall/dwgen),
//! but can be used on it's own to build other products.
//! 
//! This early version of the library is very simple, and requires command-line
//! arguments to be passed to set the required values for `Args`.

use std::io::Error;
use clap::Parser;


/// All options passed are gathered in the Args struct
/// # Example
/// ```
/// use clap::Parser;
/// let args = Args::parse;
/// ```
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Name of local file
    #[arg(short, long)]
    file: String,

    /// Number of words
    #[arg(short, long)]
    number: Option<u32>,
}

/// Pass your args to the `generate_password` function. 
/// Returns a `Vec<String>`
/// # Example:
/// ```
/// use clap::Parser;
/// let args = Args::parse();
/// let password = generate_password(args);
/// println!("Password: {}", password);
/// ```
pub fn generate_password(args: Args) -> Vec<String> {
    use rand::rngs;
    use rand::seq::{IteratorRandom, SliceRandom};

    // Ingest contens of the file and save the words in a Vec
    let mut words: Vec<String> = Vec::new();
    if let Ok(contents) = process_file(&args.file) {
        contents
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
    for _ in 0..=args.number.unwrap_or(5) {
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
