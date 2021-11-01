use clap::{App, Arg};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> Result<Config, Box<dyn std::error::Error>> {
    let matches = App::new("rust-cat")
        .version("0.1.0")
        .author("MrBanana")
        .about("Utility `cat` written by Rust")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .help("number all output lines")
                .takes_value(false)
                .conflicts_with("number_nonblank_lines"),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .short("b")
                .long("number-nonblank")
                .help("number nonempty output lines, overrides -n")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank_lines"),
    })
}

fn open(filename: &str) -> Result<Box<dyn BufRead>, Box<dyn std::error::Error>> {
    Ok(Box::new(BufReader::new(File::open(filename)?)))
}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    for filename in config.files {
        // 1. Open file
        match open(&filename) {
            Err(e) => eprintln!("{}: {:?}", filename, e),
            Ok(file) => {
                //println!("{}: ", filename);

                // 2. Display file content with specified args
                let mut nonblank_line_num: usize = 0;
                for (line_num, line_content) in file.lines().enumerate() {
                    let line_content = line_content?;
                    if config.number_lines {
                        println!("{:6}\t{}", line_num + 1, line_content);
                    } else if config.number_nonblank_lines && !line_content.is_empty() {
                        nonblank_line_num += 1;
                        println!("{:6}\t{}", nonblank_line_num, line_content);
                    } else {
                        println!("{}", line_content);
                    }
                }
            }
        }
    }

    Ok(())
}
