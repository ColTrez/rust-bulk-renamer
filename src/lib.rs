use std::ffi::OsString;
use std::fs;
use std::io;
use regex::Regex;
use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[clap(parse(from_os_str))]
    pub directory: std::path::PathBuf,
    #[clap(short, long)]
    pub replace: String,
    #[clap(default_value_t = String::from(""), short, long)]
    pub with: String,
    #[clap(long)]
    pub recurse: bool,
}

impl Args {
    pub fn read_cli_args() -> Args {
        Args::parse()
    }
}

#[derive(Debug)]
pub enum RenamerError {
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



pub fn rename_files(args: &Args) -> Result<(), RenamerError>{
    return rename_files_in_directory(&args, &args.directory);
}

fn rename_files_in_directory(args: &Args, path: &std::path::PathBuf)-> Result<(), RenamerError>{
    for entry in fs::read_dir(path)? {

        let dir = entry?;
        //handle files (non directories)
        if dir.file_type()?.is_file(){
            
            let name :String = dir.file_name().into_string()?;   
            let re = Regex::new(&args.replace)?;    
            let regex_text = name.clone();
            let new_filename = re.replace_all(regex_text.as_str(), args.with.clone());
            let path = dir.path();
            let mut rename_to = String::new();
            if let Some(path) = path.parent() {
                rename_to = path.to_owned().as_os_str().to_owned().into_string()?;
            }
            rename_to.push('/');//this wont work on windows but fuck windows for making this a problem
            rename_to.push_str(&new_filename);       
            
            fs::rename(dir.path(), rename_to)?;
        }
        //if recurse flag was used, handle directories
        else if args.recurse && dir.file_type()?.is_dir() {
            rename_files_in_directory(&args, &dir.path())?;
        }
    }
    return Ok(());
}