extern crate clap;

use std::io::{self, BufReader, StdoutLock, Read, Write};
use std::fs::File;

use clap::{Arg, App};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");


fn write_buffer(handle: &mut StdoutLock, buffer: &[u8]) {
    match handle.write(&buffer) {
        Ok(_) => (),
        Err(error) => panic!("Unable to write to stdout: {}", error),
    }
}


fn handle_input(input_file: &str) {
    let stdout = io::stdout();
    let file = File::open(input_file).unwrap();
    let mut reader = BufReader::new(file);

    // We want to lock stdout for this entire file, we don't want other data in the output.
    let mut handle = stdout.lock();
    // A 4kB buffer for input reading.
    let mut buffer = [0; 4096];

    'read: loop {
        match reader.read(&mut buffer) {
            Ok(n) => if n > 0 {
                // We pass a slice of the buffer ending at the n, which is number of bytes read
                // from the input file. Otherwise we would write the entire buffer containing
                // zeroes.
                write_buffer(&mut handle, &buffer[0 .. n])
            }
            else {
                // EOF, break read loop.
                break 'read
            },
            Err(error) => panic!("Error while reading file {}, error was: {}", input_file, error),
        }
    }
}


fn main() {
    let matches = App::new("cate")
        .version(VERSION)
        .author("Lukas Nemec <lu.nemec@gmail.com>")
        .about("cat in Rust - cate, concatenate FILE(s), or standard input, to standard output. Very naive implementation.")
        .arg(Arg::with_name("FILE")
                 .multiple(true)
                 .required(true))
    .get_matches();

    if let Some(values) = matches.values_of("FILE") {
        for input_file in values {
            handle_input(input_file);
        }
    }
}
