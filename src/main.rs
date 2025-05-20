// importing neccessary modules.
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Result};

fn main() -> Result<()> {
    // collecting arguments as a vector.
    let arguments: Vec<String> = env::args().collect();

    // this applies only if user has provided one argument, for only one file.
    if arguments.len() <= 2 {
        // using file path provided by the user.
        let file_path = arguments
            .get(1)
            .expect("no file path or incorrect file path is provided");

        // opening the file if exists
        let file = File::open(file_path).expect("file do not exists, try again");

        // reading file with bufferreader
        let reader = BufReader::new(file);

        // displaying information about starting of file.
        println!("\n------------------");
        println!("Start of the file");
        println!("------------------\n");

        // printing file line by line using for loop.
        for line in reader.lines() {
            match line {
                Ok(content) => println!("{}", content),
                Err(e) => {
                    eprintln!("failed to read from {}: {}", file_path, e);
                    break;
                }
            }
        }

        // displaying information about ending of file.
        println!("\n------------------");
        println!("End of the file");
        println!("------------------\n");
    }
    // this applies if user has provided more than one arguments, for reading more than one file.
    else if arguments.len() > 2 {
        // applying for loop to read files, one after another.
        for i in 1..arguments.len() {
            // using file path provided by user
            let file_path = match arguments.get(i) {
                Some(path) => path,
                None => {
                    eprintln!("missing or invalid file at file number - {}", i);
                    continue;
                }
            };

            // opening the file if exists
            let file = match File::open(file_path) {
                Ok(o) => o,
                Err(e) => {
                    eprintln!(
                        "failed to open file number - {} at given path - {}, {}",
                        i, file_path, e
                    );
                    continue;
                }
            };

            // reading the file with bufferreader
            let reader = BufReader::new(file);

            // displaying information about starting of file with it's number.
            println!("\n-----------------------");
            println!("Start of the file - {i}");
            println!("-----------------------\n");

            // printing file line by line using for loop.
            for line in reader.lines() {
                match line {
                    Ok(content) => println!("{}", content),
                    Err(e) => {
                        eprintln!("failed to read from {}: {}", file_path, e);
                        break;
                    }
                }
            }

            // displaying information about ending of file with it's number.
            println!("\n-----------------------");
            println!("End of the file - {i}");
            println!("-----------------------\n");
        }
    }

    // on getting successful result.
    Ok(())
}
