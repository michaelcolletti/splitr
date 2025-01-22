use std::env;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: {} <split|reassemble> <file_path> <split_size|part_count>", args[0]);
        return Ok(());
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "split" => {
            let split_size = parse_size(&args[3])?;
            split_file(file_path, split_size)?;
        }
        "reassemble" => {
            let part_count: usize = args[3].parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid part count"))?;
            reassemble_files(file_path, part_count)?;
        }
        _ => {
            eprintln!("Invalid command. Use 'split' or 'reassemble'.");
        }
    }

    Ok(())
}

fn split_file(file_path: &str, split_size: usize) -> io::Result<()> {
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
        "kb" => num * 1024,
        _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid unit")),
    };
    Ok(size)
}

fn reassemble_files(file_path: &str, part_count: usize) -> io::Result<()> {
    let reassembled_file_path = format!("{}_reassembled", file_path);
    let mut output_file = File::create(&reassembled_file_path)?;

    for part_number in 0..part_count {
        let part_path = format!("{}_part_{}", file_path, part_number);
        let mut part_file = File::open(&part_path)?;
        let mut buffer = Vec::new();
        part_file.read_to_end(&mut buffer)?;
        output_file.write_all(&buffer)?;
    }

    println!("Reassembly complete. Validating integrity...");

    if validate_integrity(file_path, &reassembled_file_path)? {
        println!("Validation successful. The reassembled file is identical to the original file.");
    } else {
        println!("Validation failed. The reassembled file differs from the original file.");
    }

    Ok(())
}

fn validate_integrity(original_file_path: &str, reassembled_file_path: &str) -> io::Result<bool> {
    let mut original_file = File::open(original_file_path)?;
    let mut reassembled_file = File::open(reassembled_file_path)?;

    let mut original_buffer = Vec::new();
    let mut reassembled_buffer = Vec::new();

    original_file.read_to_end(&mut original_buffer)?;
    reassembled_file.read_to_end(&mut reassembled_buffer)?;

    Ok(original_buffer == reassembled_buffer)
}
