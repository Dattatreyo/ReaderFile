use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    // Specify the path to the file
    let file_path = "example.txt";

    // Open the file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Read and print the file line by line
    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}