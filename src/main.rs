use file_renamer::RenamerError;

use std::io;

fn main() -> Result<(), RenamerError>{

    //task1: read arguments & set variables
    let args = file_renamer::Args::read_cli_args();

    //task2: ask for confirmation before renaming
    if args.with.len() == 0 {
        println!("Every file in directory {:?} will have any occurance of {} removed.", args.directory, args.replace);
    }
    else {
        println!("Every file in directory {:?} will have any occurance of {} replaced with {}", args.directory, args.replace, args.with);
    }
    if args.recurse {
        println!("Since you used the recursive flag, the same operation shall be done on every file in every subdirectory of {:?}", args.directory);
    }
    println!("Are you sure you want to do this? Y/N (defaults to no):");
    let mut confirm = String::new();

    io::stdin()
        .read_line(&mut confirm)
        .expect("Error: could not read confirmation response");
    
    //task3: rename
    match confirm.as_str().trim() {
        "Y" | "y" | "YES" | "Yes" | "yes" => file_renamer::rename_files(&args),
        _ => {
            println!("Understandable, have a nice day");
            Ok(())
        },
    }
}