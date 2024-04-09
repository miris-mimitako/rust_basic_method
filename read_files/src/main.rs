use std::env; // to get the arguments passed to the program
use std::fs; // to read files

fn main() {
    // Get the arguments passed to the program
    let args: Vec<String> = env::args().collect(); 
        // Vec<String> is a vector of strings
        // env::args() returns an iterator of the arguments passed to the program

    // Check if the number of arguments is correct
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        // file lists
        let files = fs::read_dir(".").unwrap();
        for file in files {
            println!("{:?}", file.unwrap().path());
        }
        return;
    }

    // Get the filename from the arguments
    let filename = &args[1];

    // Read the file
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    // Print the contents of the file
    println!("File contents:");
    println!("{}", contents)
}


