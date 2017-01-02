extern crate clap;
#[macro_use]
extern crate lazy_static;

use std::io::{self, BufReader, StdoutLock, Read, Write};
use std::fs::File;
use std::path;

use clap::{Arg, App};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

// A 64kB buffer for input reading.
const BUFFER_CAPACITY: usize = 1024 * 64;

const CATE: &'static str = "
----------- meow start -----------
       _                        
       \\`*-.                    
        )  _`-.                 
       .  : `. .                
       : _   '  \\               
       ; *` _.   `*-._          
       `-.-'          `-.       
         ;       `       `.     
         :.       .        \\    
         . \\  .   :   .-'   .   
         '  `+.;  ;  '      :   
         :  '  |    ;       ;-. 
         ; '   : :`-:     _.`* ;
      .*' /  .*' ; .*`- +'  `*' 
      `*-*   `*-*  `*-*'        

----------- meow end -----------
";

lazy_static! {
    static ref CATE_PATH: path::PathBuf = path::PathBuf::from("cate");
    static ref STDIN_PATH: path::PathBuf = path::PathBuf::from("-");
}


fn write_buffer(handle: &mut StdoutLock, buffer: &[u8]) {
    match handle.write(&buffer) {
        Ok(_) => (),
        Err(error) => panic!("cate: unable to write to stdout: {}", error.to_string()),
    }
}

fn print_cate() {
    writeln!(&mut io::stderr(), "{0}", CATE.to_string()).unwrap();
}

fn stderr(output: &str) {
    writeln!(&mut io::stderr(), "{}", output).unwrap()
}

// Tries to create Box<Read> for given `input_file`.
fn file_reader(input_file: &path::PathBuf) -> Result<Box<Read>, io::Error> {
    if input_file.is_file() {
        match File::open(input_file) {
            Ok(file) => return Ok(Box::new(BufReader::with_capacity(BUFFER_CAPACITY, file)) as Box<Read>),
            Err(error) => Err(error),
        }
    } else if input_file.is_dir() {
        Err(io::Error::new(io::ErrorKind::Other, "Is a directory"))
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "No such file or directory"))
    }
}

// Reads a reader from `input` and writes its contents to stdout. Errors are printed to stderr.
fn read_input(mut input: Box<Read>) -> Result<(), io::Error> {
    // We want to lock stdout for this entire file, we don't want other data in the output.
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let mut buffer = [0; BUFFER_CAPACITY];

    'read: loop {
        match input.read(&mut buffer) {
            Ok(n) => {
                if n > 0 {
                    // We pass a slice of the buffer ending at the n, which is number of bytes read
                    // from the input file. Otherwise we would write the entire buffer containing
                    // zeroes.
                    write_buffer(&mut handle, &buffer[0..n])
                } else {
                    // EOF, flush writer and break read loop.
                    try!(handle.flush());
                    break 'read;
                }
            }
            Err(error) => return Err(error),
        }
    }
    Ok(())
}

// Handles matching of input parameters
fn handle_input(input: &path::PathBuf) -> u8 {
    let error_part = format!("cate: {0}", input.display());

    if *input == *CATE_PATH {
        print_cate();
        return 0
    } else if *input == *STDIN_PATH {
        let stdin = io::stdin();
        match read_input(Box::new(stdin) as Box<Read>) {
            Ok(_) => return 0,
            Err(error) => {
                stderr(format!("{0}: {1}", error_part, error).as_str());
                return 1
            }
        }
    } else {
        match file_reader(input) {
            Ok(reader) => match read_input(reader) {
                Ok(_) => return 0,
                Err(error) => {
                    stderr(format!("{0}: {1}", error_part, error).as_str());
                    return 1
                }
            },
            Err(error) => {
                stderr(format!("{0}: {1}", error_part, error).as_str());
                return 1
            }
        }
    }
}


fn main() {
    let matches = App::new("cate")
        .version(VERSION)
        .author(AUTHORS)
        .about(DESCRIPTION)
        .arg(Arg::with_name("FILE")
            .multiple(true)
            .required(true)
            .help("file or - for stdin"))
        .get_matches();

    let mut exit_codes: Vec<u8> = vec!();
    if let Some(values) = matches.values_of("FILE") {
        for input in values {
            let path = path::PathBuf::from(input);
            exit_codes.push(handle_input(&path));
        }
    }

    let ok = exit_codes.iter().all(|&x| x == 0);
    if !ok {
        std::process::exit(1)
    }
}
