extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::error::Error;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct Person {
    name: String,
    discrim: String,
    is_bot: String
}


fn main() {

    // Variables
    let path = Path::new("data/data.json"); // Where you want to store the data from the cargo folder
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           why.description()),
        Ok(file) => file,
    };

    let mut data = Person { name: "".to_string(), discrim: "".to_string(), is_bot: "".to_string() };

    // Request Info - Username
    data.name = input("What is your username? ")
        .expect("Something went wrong!"); // Catch Errors
    println!("Hello, {}!", data.name); // Output User Input

    // Request Info - Discrim
    data.discrim = input("What is your discrim? ")
        .expect("Something went wrong!"); // Catch Errors
    println!("Updated to: {}#{}!", data.name, data.discrim); // Output User Input

    // Request Info - Bot
    data.is_bot = input("Are you a bot? true or false: ")
        .expect("Something went wrong!"); // Catch Errors

    if data.is_bot == "true" {
        println!("Got it, {}#{} is a bot!", data.name, data.discrim); // Output User Input
    } else {
        println!("Got it, {}#{} is not a bot!", data.name, data.discrim); // Output User Input
    }

    // Esentially a formatted JSON.stringify() in JavaScript
    let res = serde_json::to_string_pretty(&data);

    // Writes to file opened earlier
    match file.write_all(&res.ok().unwrap().as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
                                               why.description())
        },
        Ok(_) => println!("Successfully wrote to {}!", display),
    }


}

/// `input` mimics the input function in Python3
fn input(user_message: &str) -> io::Result<String> {
    use std::io::Write; // Use Trait
    print!("{}", user_message); // Add message to print buffer
    io::stdout().flush()?; // Flush buffer (output everything in print)

    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim_right().to_owned())
}
