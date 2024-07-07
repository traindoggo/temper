use home_dir::HomeDirExt;
use std::{
    fs::{self},
    io::{self, stdout, Write},
    path::{Path, PathBuf},
};

/// Get dirpath
/// when dirpath exists, return this path as Pathbuf
/// when dirpath does not exist, create ~/.config/temper
/// and return this path
pub fn find_or_create_dir(path: &str) -> PathBuf {
    let dirpath = path.expand_home().expect("something went wrong with !?");

    // create 'path' dir
    if !dirpath.exists() {
        match fs::create_dir(&dirpath) {
            Ok(_) => println!("* create templer dir"),
            Err(e) => panic!("can't create dir: {}", e),
        }
    }

    dirpath
}

/// get all dirpaths under ~/.config/temper
pub fn get_temper_dirpaths(dirpath: PathBuf) -> Vec<PathBuf> {
    let dir_entries = match fs::read_dir(dirpath) {
        Ok(entries) => entries,
        Err(e) => panic!("{}", e),
    };

    let mut dirpaths: Vec<PathBuf> = Vec::new();

    for entry in dir_entries.into_iter() {
        match entry {
            Ok(direntry) => {
                let dirpath = direntry.path();

                // TODO: more better way
                if !dirpath
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .starts_with(".")
                {
                    dirpaths.push(dirpath);
                }
            }
            Err(_) => println!("{:?}", "hoge"),
        };
    }

    if dirpaths.len() == 0 {
        panic!("no tempers dirs D:");
    }

    dirpaths.sort();
    dirpaths
}

pub fn get_userinput(size: usize) -> usize {
    loop {
        print!("\ninput num (0 - {}): ", size - 1);
        stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let idx: usize = match input.trim().parse() {
            Ok(num) => {
                if num < size {
                    num
                } else {
                    println!("* input num between ({}, {})", 0, size - 1);
                    continue;
                }
            }
            Err(_) => {
                println!("* input num between ({}, {})", 0, size - 1);
                continue;
            }
        };

        return idx;
    }
}

pub fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let filetype = entry.file_type()?;
        if filetype.is_dir() {
            copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
