use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        println!("Nice");
    }
    else if args.len() == 4 {
        println!("Good");
    }
    else {
        println!("Usage: scp [-r] <source_path> <dest_path>");
    }

    println!("{:?}", args);
}
