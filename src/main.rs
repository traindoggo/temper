use home_dir::HomeDirExt;
use std::{
    env::current_dir,
    fs,
    io::{self, stdout, Write},
    path::{Path, PathBuf},
};

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

fn main() {
    // get all dirpaths under ~/.config/temper
    let mut dirpaths: Vec<PathBuf> = Vec::new();

    let temper_entries = fs::read_dir("~/.config/temper".expand_home().unwrap()).unwrap();
    for entry in temper_entries.into_iter() {
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

    dirpaths.sort();

    println!("------------------");
    for (index, dirpath) in dirpaths.iter().enumerate() {
        println!(">>> {}. {:?}", index, dirpath.file_name().unwrap());
    }

    // get user input
    let mut _idx: usize = 0;

    loop {
        print!("\ninput num (0 - {}): ", dirpaths.len() - 1);
        stdout().flush().unwrap();

        // conversion buffer to integer
        let mut buffer = String::new();
        let _ = io::stdin().read_line(&mut buffer);

        match buffer.trim().to_string().parse::<usize>() {
            Ok(num) => {
                if num < dirpaths.len() {
                    _idx = num;
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

    // copy all target_path files to current dir
    let target_path = &dirpaths[_idx];
    let current = current_dir().expect("can't get current dir");
    let _ = copy_recursively(target_path, current);

    println!(
        "\n>>> your choice is: {:?}",
        dirpaths[_idx].file_name().unwrap()
    );
    println!(">>> success :^)");
}
