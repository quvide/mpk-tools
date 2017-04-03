use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;

// This namespace has the relevant structs and encoding/decoding functions
use mpk;

pub fn unpack(source_file_path: &str, out_dir_path: &str) {
    println!("Unpacker started!");

    println!("Opening file {}.", source_file_path);
    let mut source_file = File::open(source_file_path).expect("Couldn't open file :/");

    // Read the mpk file header to memory
    // It tells us how many "sub"file headers we should expect
    let mut header = [0; mpk::HEADER_SIZE];
    source_file.read_exact(&mut header)
            .expect("Couldn't read bytes, panicking!");
    let header = mpk::Header::decode(&header);

    println!("Archive contains {} files.", header.file_count);

    let mut files: Vec<mpk::FileHeader> = Vec::new();

    // Read all "sub"file headers to memory
    for _ in 0..header.file_count {
        let mut buf = [0; mpk::FILEHEADER_SIZE];
        source_file.read_exact(&mut buf)
                .expect("Couldn't read bytes, panicking!");

        files.push(mpk::FileHeader::decode(&buf));
    }

    println!("Read file headers.");

    // Go through every file header and copy the specified bytes to actual files
    for file in files {
        println!("Unpacking file {} of {}", file.file_index+1, header.file_count);

        // Remove null bytes from end of file path
        let mut file_path = file.file_path.to_vec();
        let mut first_null: usize = 0;
        for (idx, el) in file_path.iter().enumerate() {
            if *el == 0 {
                first_null = idx;
                break
            }
        }
        file_path.truncate(first_null);

        // Get file handle to output file
        let mut out_file = File::create(
                format!("{}/{}_{}", out_dir_path, file.file_index,
                String::from_utf8(file_path)
                        .expect("File path was not valid UTF-8. Panicking!")))
                .expect("Couldn't create output file. Panicking!");

        // Set the cursor position to the one specified in the header
        source_file.seek(SeekFrom::Start(file.begin_index))
                .expect("Invalid resource start address!");

        // Select `length` bytes forward from the cursor
        let mut source_file = (&source_file).take(file.length);

        // Read and write the selected bytes
        loop {
            let mut buf = [0 as u8; 1024*1024];

            // Read some amount of bytes to buf, returns amout of bytes read
            let n = source_file.read(&mut buf).expect("Couldn't read bytes, panicking!");

            // If we read 0 bytes, we have already read everything we should
            if n == 0 {
                break;
            }

            // Write all that we previously read to the buffer
            out_file.write(&buf[..n]).expect("Couldn't write bytes, panicking!");
        }
    }
}
