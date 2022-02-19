use std::ffi::OsString;
use std::fs;
use std::io;
use regex::Regex;
use clap::Parser;

fn main() -> Result<(), RenamerError>{

    //task1: read arguments & set variables
    let args = Args::parse();
    
    println!("Directory: {:?}", args.directory);
    println!("to_replace: {}", args.replace);
    println!("with: {:?}", args.with);
    println!("recursive_flag: {:?}", args.recurse);

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
        "Y" | "y" | "YES" | "Yes" | "yes" => {
            rename_files(&args)
        },
        _ => {
            println!("Understandable, have a nice day");
            Ok(())
        },
    }
}

#[derive(Parser)]
struct Args {
    #[clap(parse(from_os_str))]
    directory: std::path::PathBuf,
    #[clap(short, long)]
    replace: String,
    #[clap(default_value_t = String::from(""), short, long)]
    with: String,
    #[clap(long)]
    recurse: bool,
}

#[derive(Debug)]
enum RenamerError {
    IO(io::Error),
    OsString(OsString),
    Regex(regex::Error),
}

impl From<io::Error> for RenamerError {
    fn from(err: io::Error) -> RenamerError {
        RenamerError::IO(err)
    }
}

impl From<OsString> for RenamerError {
    fn from(err: OsString) -> RenamerError {
        RenamerError::OsString(err)
    }
}

impl From<regex::Error> for RenamerError {
    fn from(err: regex::Error) -> RenamerError {
        RenamerError::Regex(err)
    }
}

fn rename_files(args: &Args) -> Result<(), RenamerError>{
    return rename_files_in_directory(&args, &args.directory);
}

fn rename_files_in_directory(args: &Args, path: &std::path::PathBuf)-> Result<(), RenamerError>{
    for entry in fs::read_dir(path)? {

        let dir = entry?;
        println!("{:?}", dir.path());
        //handle files (non directories)
        if dir.file_type()?.is_file(){
            println!("1");
            let name :String = dir.file_name().into_string()?;
            println!("2");
            let re = Regex::new(&args.replace)?;
            println!("3");
            let regex_text = name.clone();
            println!("4");
            let new_filename = re.replace_all(regex_text.as_str(), args.with.clone());
            println!("5");
            println!("name: {}", name);
            //fs::rename(name, new_filename.into_owned())?;
            //fs::rename(dir.path(), new_filename.into_owned())?;
            let path = dir.path();
            let mut rename_to = String::new();
            if let Some(path) = path.parent() {
                rename_to = path.to_owned().as_os_str().to_owned().into_string()?;
            }
            rename_to.push('/');//this wont work on windows but fuck windows for making this a problem
            rename_to.push_str(&new_filename);       
            println!("rename to: {}", rename_to);
            //fs::copy(dir.path(), rename_to)?;
            fs::rename(dir.path(), rename_to)?;
            println!("6");
        }
        //if recurse flag was used, handle directories
        else if args.recurse && dir.file_type()?.is_dir() {
            rename_files_in_directory(&args, &dir.path())?;
        }
    }
    return Ok(());
}