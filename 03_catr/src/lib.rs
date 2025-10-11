use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {

    #[arg(value_name = "FILE", help = "Input file(s)", default_value = "-")]
    files: Vec<String>,

    #[arg(short, long = "number", group = "numbering", help = "Number lines", default_value_t = false)]
    number_lines: bool,

    #[arg(short = 'b', long ="number-nonblank", group = "numbering", help = "Number non-blank lines")]
    number_nonblank_lines: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(buf) => {
                let mut line_number = 1;
                for line in buf.lines() {
                    let line = line?;
                    if config.number_lines || (config.number_nonblank_lines && !line.is_empty()) {
                        print!("{:>6}\t", line_number);
                        line_number += 1;
                    }
                    println!("{}", line)
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn get_args() -> MyResult<Config> {
    Ok(Config::parse())
}
