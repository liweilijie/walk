use std::env;
use std::io::Error;
use std::path::Path;

// 下面一行相当于注释的两行
use walkdir::WalkDir;

fn walk<P: AsRef<Path>>(path: P) -> Result<(), Error> {
    let mut count = 0;
    for entry in WalkDir::new(path) {
        // let entry = entry?; // TODO:
        let entry = match entry {
            Err(e) => {
                println!("happened error :{}.", e);
                continue;
            },
            Ok(entry) => entry,
        };

        if entry.file_type().is_dir() {
            continue;
        }
        let name = entry.file_name();
        if let Some(s) = name.to_str() {
            if s.to_ascii_lowercase().ends_with(".jpg") {
                count += 1;
            }
        }
    }
    println!("{}", count);
    Ok(())
}

fn main() {
    println!("Hello, Walk dir !");
    if let Err(e) = walk(env::args().nth(1).unwrap()) {
        println!("error: {}", e);
    }

    /*
    等同于这样:
    match walk(env::args().nth(1).unwrap()) {
        Err(e) => println!("error {}", e),
        Ok(_) => {},
    }
    */
}

