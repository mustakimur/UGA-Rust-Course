use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::ErrorKind;
use std::io::Read;
use std::io::{BufRead, BufReader, Write};
use std::{env, fs};

fn main() {
    //panic_msg();

    //file_01();

    //file_02();

    //file_03();

    //file_04();

    //file_append_write();

    //file_05();

    //directory_traversal();

    file_guard_example();
}

fn panic_msg() {
    let v = vec![1, 2, 3];

    panic!("Let's create a panic!!!");
    //v[99];
}

fn file_01() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

fn file_02() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}

fn file_03() {
    let f = File::open("hello2.txt").unwrap();
    //let f = File::open("hello2.txt").expect("Failed to open hello.txt");
}

fn file_04() {
    match read_username_from_file() {
        Ok(str) => println!("Message Received: {}", str),
        Err(e) => eprintln!("There is an error of {}", e),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

/* fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
} */

fn file_append_write() {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("hello.txt")
        .unwrap();

    if let Err(e) = writeln!(file, "A new line!") {
        eprintln!("Couldn't write to file: {}", e);
    }
}

fn file_05() -> Result<(), io::Error> {
    let path = "lines.txt";

    let mut output = File::create(path)?;
    write!(output, "Rust\nis\nFun")?;

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line?);
    }

    /* let mut s = String::new();

    input.read_to_string(&mut s)?;

    println!("{}", s); */

    Ok(())
}

fn directory_traversal() -> Result<(), Box<dyn Error>> {
    let current_dir = env::current_dir()?;
    println!(
        "Entries modified in the last 24 hours in {:?}:",
        current_dir
    );

    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();

        let metadata = fs::metadata(&path)?;
        let last_modified = metadata.modified()?.elapsed()?.as_secs();

        if last_modified < 24 * 3600 && metadata.is_file() {
            println!(
                "Last modified: {:?} seconds, is read only: {:?}, size: {:?} bytes, filename: {:?}",
                last_modified,
                metadata.permissions().readonly(),
                metadata.len(),
                path.file_name().ok_or("No filename")?
            );
        }
    }

    Ok(())
}

use file_guard::Lock;
use std::thread;
use std::time::Duration;

fn file_guard_example() -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("hello.txt")?;

    let mut num = 0;
    loop {
        let mut lock = file_guard::lock(&mut file, Lock::Exclusive, 0, 1)?;
        let str = format!("{}-{}", "hello lock", num);
        writeln!(&mut lock, "{}", str)?;
        thread::sleep(Duration::from_secs(2));
        num += 1;
    }
}
