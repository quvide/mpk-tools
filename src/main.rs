extern crate clap;
extern crate byteorder;

mod unpacker;
mod packer;
mod mpk;
mod util;

use util::directory_validate;
use clap::{App, SubCommand};

fn main() {
    let matches = App::new("mpk-tools")
            .version("0.1.0")
            .author("Elias B. <elias.benkhodja@gmail.com>")
            .about("Unpack mpk archives")
            .subcommand(SubCommand::with_name("unpack")
                    .about("Extracts files from the archive")
                    .args_from_usage("-o, --output=<DIR> 'Sets output directory'
                                     <FILE> 'The archive file'"))
            .subcommand(SubCommand::with_name("pack")
                    .about("Extracts files from the archive")
                    .args_from_usage("-o, --output=<FILE> 'Sets output file'
                                     <DIR> 'Directory with prefixed files'"))
            .get_matches();


    if let Some(ref matches) = matches.subcommand_matches("unpack") {
        let source_file = matches.value_of("FILE").unwrap();
        let out_dir = matches.value_of("output").unwrap();

        // Check if the desired output directory exists, create if doesn't
        directory_validate(out_dir, true);

        println!("Starting unpacker.");

        unpacker::unpack(source_file, out_dir);
    }

    else if let Some(ref matches) = matches.subcommand_matches("pack") {
        let source_dir = matches.value_of("DIR").unwrap();
        let out_file = matches.value_of("output").unwrap();

        // Check if the desired input directory exists
        directory_validate(source_dir, false);

        println!("Starting packer.");

        packer::pack(source_dir, out_file);
    }

    println!("Done!");
}
