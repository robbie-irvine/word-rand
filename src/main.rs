use std::{vec, io};
use std::string::String;
use fastrand::usize;

fn main() {
    let mut words: Vec<String> = vec![];

    println!("Type in some words/sentences to be randomised.\nPress return to divide, leave blank to stop\n---");
    
    'inputs: loop {
        let mut current_word = String::new();
        io::stdin().read_line(&mut current_word).expect("failed to read string");

        let temp: &str = current_word.as_str().trim();

        if temp.eq("") {
            break 'inputs;
        }

        words.push(temp.to_string());
    }

    if words.len() > 0 {
        let i = usize(..words.len());
        println!("{}", words[i]);
    } else {
        println!("No values were entered");
    }
}
