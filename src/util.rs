use std::fs::{metadata, create_dir};

pub fn directory_validate(dir: &str, create: bool) {
    println!("Checking if directory {} exists.", dir);
    let meta = metadata(dir);
    let mut ok: bool = false;
    match meta {
        Ok(meta) => {
            if meta.is_dir() {
                ok = true;
            }
        },
        Err(_) => {}
    }

    if !create && !ok {
        panic!("Directory not found.");
    } else if create && !ok {
        create_dir(dir).expect("Couldn't create directory.");
        println!("Directory created.");
    } else {
        println!("Found directory.")
    }
}

pub fn magic_padding(pos: u64) -> u64 {
    println!("{}", pos);
    return (2048 * (pos as f64 / 2048.0).ceil() as u64) - pos;
}
