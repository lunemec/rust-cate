extern crate clap;

use std::io::{self, BufReader, StdoutLock, Read, Write};
use std::fs::File;

use clap::{Arg, App};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

// A 64kB buffer for input reading.
const BUFFER_CAPACITY: usize = 1024 * 64;


fn write_buffer(handle: &mut StdoutLock, buffer: &[u8]) {
    match handle.write(&buffer) {
        Ok(_) => (),
        Err(error) => panic!("Unable to write to stdout: {}", error),
    }
}


fn get_reader(input_file: &str) -> Option<Box<Read>> {
    if input_file == "-" {
        let stdin = io::stdin();
        return Some(Box::new(stdin) as Box<Read>);
    } else {
        match File::open(input_file) {
            Ok(file) => return Some(Box::new(BufReader::with_capacity(BUFFER_CAPACITY, file)) as Box<Read>),
            Err(error) => {
                writeln!(&mut io::stderr(), "cate: {0}: {1}", input_file, error.to_string()).unwrap();
                None
            }
        }
    }
}


fn handle_input(input_file: &str) {
    let stdout = io::stdout();
    if let Some(mut reader) = get_reader(input_file) {
        // We want to lock stdout for this entire file, we don't want other data in the output.
        let mut handle = stdout.lock();
        let mut buffer = [0; BUFFER_CAPACITY];

        'read: loop {
            match reader.read(&mut buffer) {
                Ok(n) => if n > 0 {
                    // We pass a slice of the buffer ending at the n, which is number of bytes read
                    // from the input file. Otherwise we would write the entire buffer containing
                    // zeroes.
                    write_buffer(&mut handle, &buffer[0 .. n])
                }
                else {
                    // EOF, flush writer and break read loop.
                    match handle.flush() {
                        Ok(_) => {},
                        Err(error) => writeln!(&mut io::stderr(), "cate: {0}: {1}", input_file, error.to_string()).unwrap(),
                    }
                    break 'read;
                },
                Err(error) => panic!("Error while reading file {}, error was: {}", input_file, error),
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

    if let Some(values) = matches.values_of("FILE") {
        for input_file in values {
            handle_input(input_file);
        }
    }
}
