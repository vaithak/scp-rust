extern crate walkdir;
use std::env;
use std::process;
use walkdir::WalkDir;
use std::fs;

// -> io::Result<()>
// Recursively copying folders => currently not implemented
// fn recursive_copy(source_path: String, dest_path: String) {
//     for entry in WalkDir::new(source_path).follow_links(true) {
//         let entry = entry.unwrap();
//         println!("{}", entry.path().display());
//     }
// }

// Function to simply copy a file
fn simple_copy(source_path: String, dest_path: String) -> std::io::Result<()>{
    fs::copy(source_path, dest_path);  // Copy foo.txt to bar.txt
    Ok(())
}

// Checking if given path string is a file
fn check_type(file_type: &String) -> i32{
    match fs::metadata(&file_type) {
        Ok(file) => {
            if file.file_type().is_dir() {
                0
            } else if file.file_type().is_file() {
                1
            } else {
                -1
            } 
        },
        Err(_) => -1,
    }
}

fn main() {


    let mut args: Vec<String> = env::args().collect();
    let (source_path,dest_path) ;
    if args.len() == 3 {
        source_path = args.remove(1);
        dest_path = args.remove(1);
        let check_file = check_type(&source_path);
        if check_file != 1 {
            println!("Source file not found or you don't have the permission");
        } else {
            match simple_copy(source_path, dest_path) {
                Ok(_) => println!("Done"),
                Err(_) => println!("Some error occured"),
            }
        }
    }
    else if args.len() == 4 {
        let index = args.iter().position(|&ref x| x == "-r");
        if index == Some(1){
            source_path = args.remove(2);
            dest_path = args.remove(2);
            recursive_copy(source_path, dest_path);
        } else {
            println!("Usage: scp [-r] <source_path> <dest_path>");
            process::exit(0);
        }
    }
    else {
        println!("Usage: scp [-r] <source_path> <dest_path>");
        process::exit(0);
    }
}
