use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    // Specify the path to your file
    let file_path = Path::new("input.txt");

    // Open the file
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    // Iterate over each line
    for line_result in reader.lines() {
        let line = line_result?; // Handle potential errors during line reading

        // Tokenize the line by splitting on whitespace
        let tokens: Vec<&str> = line.split_whitespace().collect();

        // Print the original line and its tokens
        println!("Original Line: \"{}\"", line);
        println!("Tokens: {:?}", tokens);
        println!("---"); // Separator for clarity
    }

    Ok(())
}

