// Import standard library
use std::fs;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};

// Set path to the file
const PATH: &str = "./shoppinglist.txt";

// function to read from file
fn read() -> Result<String, Error>{
    // Open file and read to string
    return Ok(fs::read_to_string(PATH)?);
}

// Function to write to file
fn write(input: &str) -> Result<(), Error> {
    // Try to create the file
    let mut output = File::create(PATH)?;
    // Try to write input to file
    write!(output, "{}", input)?;
    // Try to set input to file
    let input = File::open(PATH)?;
    // Read input to string
    let buffered = BufReader::new(input);

    // Try to print the file contents
    for words in buffered.lines() {
        println!("{}", words?);
    }

    // Return Ok
    Ok(())
}


// Function to remove from file
fn remove(input: i32, li: &str) -> Result<String, Error> {
    // Create a Split struct from the li input
    let list = li.split("\n");
    // Create a 32 bit signed integer with 1 as initial value
    let mut x = 1;
    // Create a String to hold the new file contents
    let mut out = "".to_string();
    // Loop through the Split struct
    for i in list {
        // If the current line is the line to remove
        if x == input {
            // Create a String with the contents to remove
            let rp = ("{}\n".to_string()).replace("{}", &i);
            // Remove the line from the file
            out = li.replace(&rp, "");
            // Break the loop
            break;
        }
        // Increase x by one and check again
        x += 1;
    }

    // Return the new file contents
    Ok(out)
}

// Create run function to pass to the main file
pub fn run() {
    println!("Shopping List");
    while true {
        let mut list: String;
        if read().is_err() {
            println!("File not found, write to create file.");
            list = "".to_string();
        } else {
            list = read().unwrap();
        }
        let mut input = String::new();
        println!("Enter command: ");
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "add" => {
                println!("Enter item: ");
                let mut item = String::new();
                std::io::stdin().read_line(&mut item).unwrap();
                list.push_str(&item);
                println!("----------- Shopping List -----------");
                write(&list).unwrap();
                println!("------------------------------------");
            },
            "remove" => {
                println!("Enter item: ");
                let mut item = String::new();
                std::io::stdin().read_line(&mut item).unwrap();
                let int = item.trim().parse::<i32>().unwrap();
                list = remove(int, &list).unwrap();
                println!("----------- Shopping List -----------");
                write(&list).unwrap();
                println!("------------------------------------");
            },
            "list" => {
                println!("----------- Shopping List -----------");
                println!("{}", list);
                println!("------------------------------------");
            },
            "exit" => {
                break;
            },
            _ => {
                println!("Invalid command");
            }
        }
    }
}
