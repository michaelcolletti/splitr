use std::env;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <file_path> <split_size>", args[0]);
        return Ok(());
    }

    let file_path = &args[1];
    let split_size = parse_size(&args[2])?;

    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = vec![0; split_size];
    let mut part_number = 0;

    while let Ok(bytes_read) = reader.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        let part_path = format!("{}_part_{}", file_path, part_number);
        let mut part_file = File::create(&part_path)?;
        part_file.write_all(&buffer[..bytes_read])?;

        part_number += 1;
    }

    Ok(())
}

fn parse_size(size_str: &str) -> io::Result<usize> {
    let size_str = size_str.to_lowercase();
    let (num, unit) = size_str.split_at(size_str.len() - 2);
    let num: usize = num.parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid size"))?;
    let size = match unit {
        "tb" => num * 1024 * 1024 * 1024 * 1024,
        "gb" => num * 1024 * 1024 * 1024,
        "mb" => num * 1024 * 1024,
        _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid unit")),
    };
    Ok(size)
}
