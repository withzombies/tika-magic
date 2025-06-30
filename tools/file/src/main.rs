use clap::{Arg, ArgAction, Command};
use std::path::Path;
use std::process;

fn main() {
    // Define command line arguments using clap
    let matches = Command::new("tika-magic-file")
        .version("1.0")
        .about("Determines the MIME type of a file by examining its contents")
        .arg(
            Arg::new("file")
                .help("The file to analyze")
                .required(true)
                .index(1)
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("all")
                .long("all")
                .short('a')
                .help("Show all potential matching MIME types instead of just the best match")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    // Get the file path
    let file_path = matches.get_one::<String>("file").unwrap();
    let path = Path::new(file_path);

    // Validate that the file exists
    if !path.exists() {
        eprintln!("Error: File '{file_path}' does not exist");
        process::exit(1);
    }

    // Check if it's a regular file
    if !path.is_file() {
        eprintln!("Error: '{file_path}' is not a regular file");
        process::exit(1);
    }

    // Get the MIME type of the file
    if matches.get_flag("all") {
        // Get all potential matching MIME types
        match tika_magic::from_filepath_exhaustive(path) {
            Some(mime_types) => {
                if mime_types.is_empty() {
                    println!("{file_path}: Could not determine file type");
                } else {
                    println!("{}: {}", file_path, mime_types.join(", "));
                }
            }
            None => println!("{file_path}: Could not determine file type"),
        }
    } else {
        // Get the single best matching MIME type
        match tika_magic::from_filepath(path) {
            Some(mime_type) => println!("{file_path}: {mime_type}"),
            None => println!("{file_path}: Could not determine file type"),
        }
    }
}
