use std::env::current_dir;
use tmper::{copy_recursively, find_or_create_dir, get_temper_dirpaths, get_userinput};

fn main() {
    let dirpath = find_or_create_dir("~/.config/temper");
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
