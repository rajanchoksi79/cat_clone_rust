/*

This is implementation of cat program of linux in Rust.
so basically it will read file if exists of any given directory, provided filename as argument and print the content of it.

Author: Rajan Choksi
Date: 19/05/2025

*/

// importing neccessary modules.
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Result};

fn main() -> Result<()> {
    // collecting arguments as a vector.
    let arguments: Vec<String> = env::args().collect();

    // using file name provided by the user.
    let file_name = arguments
        .get(1)
        .expect("no file path or incorrect file path is provided");

    // opening the file if exists
    let file = File::open(file_name).expect("file do not exists, try again");

    // reading file with bufferreader
    let reader = BufReader::new(file);

    // printing file line by line using for loop.
    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}
