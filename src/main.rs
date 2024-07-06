use home_dir::HomeDirExt;
use std::{
    env::current_dir,
    fs::{self},
    io::{self, stdout, Write},
    path::{Path, PathBuf},
};

fn main() {
    let dirpath: PathBuf = find_or_create_dir("~/.config/temper");
    let temper_dirpaths = get_temper_dirpaths(dirpath);

    println!("------------------");
    for (index, dirpath) in temper_dirpaths.iter().enumerate() {
        println!(">>> {}. {:?}", index, dirpath.file_name().unwrap());
    }

    // get user input
    let idx: usize = get_userinput(temper_dirpaths.len());

    // copy all srcs to dest
    let src = &temper_dirpaths[idx];
    let dest = current_dir().expect("can't get current dir");
    let _ = copy_recursively(src, dest);

    println!(
        "\n>>> your choice is: {:?}",
        temper_dirpaths[idx].file_name().unwrap()
    );
    println!(">>> success :^)");
}

/// Get dirpath
/// when dirpath exists, return this path as Pathbuf
/// when dirpath does not exist, create ~/.config/temper
/// and return this path
fn find_or_create_dir(path: &str) -> PathBuf {
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
fn get_temper_dirpaths(dirpath: PathBuf) -> Vec<PathBuf> {
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

/// get user input
/// the user selects the template they want to create,
/// and return the template index.
fn get_userinput(size: usize) -> usize {
    let mut idx: usize = 0;

    loop {
        print!("\ninput num (0 - {}): ", size - 1);
        stdout().flush().unwrap();

        // conversion buffer to integer
        let mut buffer = String::new();
        let _ = io::stdin().read_line(&mut buffer);

        match buffer.trim().to_string().parse::<usize>() {
            Ok(num) => {
                if num < size {
                    idx = num;
                    break;
                } else {
                    println!("huge number, again...")
                }
            }
            Err(_) => {
                println!("wrong number, again...");
                continue;
            }
        }
    }

    idx
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
