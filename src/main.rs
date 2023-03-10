use std::fs::File;
use std::io::{BufReader, Seek};
mod hxa;

fn main() {
    println!("Opening file...");


    let mut input = BufReader::new(
        File::open("untitled.hxa")
        .expect("Failed to open file")
    );

    let mut hxa_file = hxa::HXAFile::new();
    hxa_file.read_header(&mut input);
    //hxa_file.read_node(&mut input);

    dbg!(hxa_file);


    let file_location = input.seek(std::io::SeekFrom::Current (0)).expect("Could not get current position!");
    println!("{:X?}",file_location);
    dbg!(file_location);

    println!("Done");
}
