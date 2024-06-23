use home_dir::HomeDirExt;
use std::{
    env::current_dir,
    fs,
    io::{self, stdout, Write},
    path::PathBuf,
};

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
    let current = current_dir().expect("can't get current dir");
    let target_path = &dirpaths[_idx];

    for entry in fs::read_dir(target_path).expect("hoge") {
        if let Ok(entry) = entry {
            let path = entry.path();
            println!(
                "{:?} {:?}",
                path.display().to_string(),
                current.display().to_string()
            );

            let current_path = current.join(path.file_name().unwrap());

            let res = fs::copy(path.display().to_string(), current_path);

            println!("{:?}", res);
        }
    }
}
