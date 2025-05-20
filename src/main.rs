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

    // this applies only if user has provided one argument, for only one file.
    if arguments.len() < 2 {
        // using file path provided by the user.
        let file_path = arguments
            .get(1)
            .expect("no file path or incorrect file path is provided");

        // opening the file if exists
        let file = File::open(file_path).expect("file do not exists, try again");

        // reading file with bufferreader
        let reader = BufReader::new(file);

        // printing file line by line using for loop.
        for line in reader.lines() {
            println!("{}", line?);
        }
    }
    // this applies if user has provided more than one arguments, for reading more than one file.
    else if arguments.len() > 1 {
        // applying for loop to read files, one after another.
        for i in 1..arguments.len() {
            // displaying information about starting of file with it's number.
            println!("");
            println!("------------------");
            println!("File number - {i}");
            println!("------------------");
            println!("");

            // using file path provided by user
            let file_path = arguments
                .get(i)
                .expect("no file path or incorrect file path is provided");

            // opening the file if exists
            let file = File::open(file_path).expect("file do not exists, try again");

            // reading the file with bufferreader
            let reader = BufReader::new(file);

            // printing file line by line using for loop.
            for line in reader.lines() {
                println!("{}", line?);
            }

            // displaying information about ending of file with it's number.
            println!("");
            println!("------------------");
            println!("End of file - {i}");
            println!("------------------");
            println!("");
        }
    }

    // on getting successful result.
    Ok(())
}
