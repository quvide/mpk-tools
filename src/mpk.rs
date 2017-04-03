use byteorder::{LittleEndian, ByteOrder};

pub const HEADER_SIZE: usize = 68;
pub struct Header {
    pub magic: [u8; 8],
    pub file_count: u64,
    pub padding: [u8; 52]
}

impl Header {
    pub fn new(file_count: u64) -> Header {
        Header {
            magic: [0x4d, 0x50, 0x4b, 0x00, 0x00, 0x00, 0x02, 0x00],
            file_count: file_count,
            padding: [0; 52]
        }
    }

    pub fn decode(src: &[u8; HEADER_SIZE]) -> Header {
        // Not sure if there's a better way to do this...
        let mut magic = [0; 8];
        let file_count = LittleEndian::read_u64(&src[8..16]);
        let mut padding = [0; 52];

        magic.clone_from_slice(&src[..8]);
        padding.clone_from_slice(&src[16..]);

        let res = Header {magic: magic, file_count: file_count, padding: padding};

        res
    }

    pub fn encode(&self) -> [u8; HEADER_SIZE] {
        let mut res = [0; HEADER_SIZE];

        (&mut res[..8]).clone_from_slice(&self.magic);
        LittleEndian::write_u64(&mut res[8..16], self.file_count);
        (&mut res[16..]).clone_from_slice(&self.padding);

        res
    }
}

pub const FILEHEADER_SIZE: usize = 256;
pub struct FileHeader {
    pub file_index: u32,
    pub begin_index: u64,
    pub length: u64,
    pub length2: u64,
    pub file_path: [u8; 228]
}

impl FileHeader {
    pub fn decode(src: &[u8; FILEHEADER_SIZE]) -> FileHeader {
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

    pub fn encode(&self) -> [u8; FILEHEADER_SIZE] {
        let mut res = [0; FILEHEADER_SIZE];

        LittleEndian::write_u32(&mut res[..4], self.file_index);
        LittleEndian::write_u64(&mut res[4..12], self.begin_index);
        LittleEndian::write_u64(&mut res[12..20], self.length);
        LittleEndian::write_u64(&mut res[20..28], self.length2);
        (&mut res[28..]).clone_from_slice(&self.file_path);

        res
    }
}
