use byteorder::{LittleEndian, ByteOrder};

pub const HEADER_SIZE: u64 = 68;
pub struct Header {
    pub magic: [u8; 8],
    pub file_count: u64,
    pub padding: [u8; 52]
}

impl Header {
    pub fn decode(src: &Vec<u8>) -> Header {
        if src.len() as u64 != HEADER_SIZE {
            panic!("Incorrect length for header! Aborting...");
        }

        // Not sure if there's a better way to do this...
        let mut magic = [0; 8];
        let file_count = LittleEndian::read_u64(&src[8..16]);
        let mut padding = [0; 52];

        magic.clone_from_slice(&src[..8]);
        padding.clone_from_slice(&src[16..]);

        let res = Header {magic: magic, file_count: file_count, padding: padding};

        res
    }
}

pub const FILEHEADER_SIZE: u64 = 256;
pub struct FileHeader {
    pub file_index: u32,
    pub begin_index: u64,
    pub length: u64,
    pub length2: u64,
    pub file_path: [u8; 228]
}

impl FileHeader {
    pub fn decode(src: &Vec<u8>) -> FileHeader {
        if src.len() as u64 != FILEHEADER_SIZE {
            panic!("Incorrect length for header! Aborting...");
        }

        // Not sure if there's a better way to do this...
        let file_index = LittleEndian::read_u32(&src[0..4]);
        let begin_index = LittleEndian::read_u64(&src[4..12]);
        let length = LittleEndian::read_u64(&src[12..20]);
        let length2 = LittleEndian::read_u64(&src[20..28]);
        let mut file_path = [0; 228];

        file_path.clone_from_slice(&src[28..]);

        let res = FileHeader {
            file_index: file_index,
            begin_index: begin_index,
            length: length,
            length2: length2,
            file_path: file_path
        };

        res
    }
}
