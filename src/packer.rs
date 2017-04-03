use std::fs::{File, read_dir};
use std::io::prelude::*;
use std::io::SeekFrom;

// This namespace has the relevant structs and encoding/decoding functions
use mpk;

use util::magic_padding;

pub fn pack(source_dir_path: &str, out_file_path: &str) {
    println!("Packer started!");

    println!("Creating file {}.", out_file_path);
    let mut out_file = File::create(out_file_path).expect("Couldn't open file :/");

    // Get all files in the directory
    let mut files: Vec<String> = Vec::new();
    let file_names = read_dir(source_dir_path).expect("Couldn't get file listing :/");
    for file in file_names {
        let file = file.expect("Couldn't read files :/").file_name();
        files.push(file.into_string().unwrap());
    }

    // Sort files by the index prefix
    files.sort_by_key(
        |file| file.split('_').next()
                       .expect("Invalid file_name!")
                       .parse::<u32>()
                       .expect("Invalid index!")
    );

    // Construct the header and write to the file
    let header = mpk::Header::new(files.len() as u64).encode();
    out_file.write(&header).expect("Couldn't write to file");

    // Skip the known-size file header part for now
    let header_len: u64 = (mpk::HEADER_SIZE + mpk::FILEHEADER_SIZE * files.len()) as u64;
    let mut pos = out_file.seek(SeekFrom::Start((header_len + magic_padding(header_len)))).unwrap();

    // Initialize FileHeader vector
    let mut file_headers: Vec<mpk::FileHeader> = Vec::new();

    // Write all files and record offsets + lengths to the vector
    for (idx, file_name) in files.iter().enumerate() {
        let file_path = format!("{}/{}", source_dir_path, file_name);
        println!("Packing file {}, {}/{}", file_path, idx+1, files.len());
        let mut file = File::open(file_path).expect("Couldn't open file for reading.");
        let begin_index = pos;

        // pad to the nearest 2048
        let padding_vec = vec![0; magic_padding(pos) as usize];
        out_file.write_all(&padding_vec).expect("Couldn't write padding bytes, panicking!");
        pos += padding_vec.len() as u64;

        loop {
            let mut buf = [0 as u8; 1024*1024];

            // Read some amount of bytes to buf, returns amout of bytes read
            let n = file.read(&mut buf).expect("Couldn't read bytes, panicking!");

            // If we read 0 bytes, we have already read everything we should
            if n == 0 {
                break;
            }

            // Write all that we previously read to the buffer
            pos += out_file.write(&buf[..n]).expect("Couldn't write bytes, panicking!") as u64;
        }

        // Remove index from file name and pad with 0
        let mut file_path = [0; 228];
        let mut file_name = file_name.splitn(2, '_').last().expect("Invalid filename, expected index prefix.").as_bytes().to_vec();
        println!("Actual filename is {}", String::from_utf8(file_name.to_owned()).unwrap());
        file_name.resize(228, 0);
        file_path.clone_from_slice(&file_name);

        file_headers.push( mpk::FileHeader {
            file_index: idx as u32,
            begin_index: begin_index as u64,
            length: file.metadata().unwrap().len(),
            length2: file.metadata().unwrap().len(),
            file_path: file_path
        });
    }

    // Now we write the file headers
    out_file.seek(SeekFrom::Start(mpk::HEADER_SIZE as u64)).unwrap();
    for header in file_headers {
        out_file.write(&header.encode()).expect("Couldn't write bytes!");
    }
}
