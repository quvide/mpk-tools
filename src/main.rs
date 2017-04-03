extern crate clap;
extern crate byteorder;

mod unpacker;
mod mpk;

use clap::{App, SubCommand};
use std::fs::{File, metadata};

fn main() {
    let matches = App::new("mpk-tools")
            .version("0.1.0")
            .author("Elias B. <elias.benkhodja@gmail.com>")
            .about("Unpack mpk archives")
            .subcommand(SubCommand::with_name("unpack")
                    .about("Extracts files from the archive")
                    .args_from_usage("-o, --output=[FILE] 'Sets output directory'
                                     <FILE> 'The archive file'"))
            .get_matches();


    if let Some(ref matches) = matches.subcommand_matches("unpack") {
        // Try to get a file handle to the archive file
        let path: &str = matches.value_of("FILE").unwrap();
        println!("Opening file {}.", path);
        let mut file = File::open(path).expect("Couldn't open file :/");

        // Check if the desired output directory exists
        let out_dir: &str = matches.value_of("output").unwrap();
        println!("Checking if directory {} exists.", out_dir);
        let meta = metadata(out_dir).expect("Couldn't open directory :/");

        if meta.is_dir() {
            println!("Named directory was found.");
        } else {
            panic!("No such directory exists")
        }

        println!("Starting unpacker.");

        unpacker::unpack(&mut file, out_dir);
    }

    println!("Done!")
}
