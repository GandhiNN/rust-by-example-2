use std::fs::File;
use std::io::Read;
use std::io::{self, BufRead, BufReader};

const FILENAME: &str = "word_frequency.txt";

fn get_count_from_line(s: &str) -> u64 {
    if s.is_empty() {
        return 0;
    }
    let mut parts = s.split_ascii_whitespace();
    let _ = parts.next();
    parts.next().unwrap().parse::<u64>().unwrap()
}

pub fn read_unbuffered_one_character_at_a_time() -> io::Result<u64> {
    let mut file = File::open(FILENAME)?;
    let len = file.metadata().expect("Failed to get file metadata").len() as usize; // get file length in bytes
    let mut v: Vec<u8> = Vec::new(); // create vector `v` of unsigned int8
    v.resize(len, 0u8); // resize the vector `v` so it has room as big as `len`
    for index in 0..len {
        file.read_exact(&mut v[index..(index + 1)])?; // read one character at a time
    }
    let s = String::from_utf8(v).expect("file is not UTF-8?");
    let mut total = 0u64;
    for line in s.lines() {
        total += get_count_from_line(line);
    }
    Ok(total)
}

pub fn read_buffered_allocate_string_every_time() -> io::Result<u64> {
    let file = File::open(FILENAME)?;
    let reader = BufReader::new(file);
    let mut total = 0u64;
    for line in reader.lines() {
        let s = line?;
        total += get_count_from_line(&s);
    }
    Ok(total)
}

pub fn read_buffered_reuse_string() -> io::Result<u64> {
    let file = File::open(FILENAME)?;
    let mut reader = BufReader::new(file);
    let mut string = String::new();
    let mut total = 0u64;
    while reader.read_line(&mut string).unwrap() > 0 {
        total += get_count_from_line(&string);
        string.clear();
    }
    Ok(total)
}

pub fn read_buffer_whole_string_into_memory() -> io::Result<u64> {
    let mut file = File::open(FILENAME)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    let mut total = 0u64;
    for line in s.lines() {
        total += get_count_from_line(line);
    }
    Ok(total)
}

