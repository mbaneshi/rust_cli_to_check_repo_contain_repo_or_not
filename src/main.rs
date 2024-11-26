use git2::Repository;
use std::env;
use std::path::Path;

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: check_git_repo <path>");
        std::process::exit(1);
    }

    let path = &args[1];
    let path_obj = Path::new(path);

    if !path_obj.exists() {
        eprintln!("The specified path '{}' does not exist.", path);
        std::process::exit(1);
    }

    if !path_obj.is_dir() {
        eprintln!("The specified path '{}' is not a directory.", path);
        std::process::exit(1);
    }

    match Repository::open(path_obj) {
        Ok(_) => println!("The directory '{}' is a Git repository.", path),
        Err(_) => println!("The directory '{}' is not a Git repository.", path),
    }
}
