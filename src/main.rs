use hxa;

fn main() {
    println!("Opening file...");

    let my_hxa = hxa::HXAFile::from("sample_models/demo.hxa");

    dbg!(my_hxa);

    // let file_location = input.seek(std::io::SeekFrom::Current (0)).expect("Could not get current position!");
    // println!("{:X?}",file_location);
    // dbg!(file_location);

    println!("Done");
}
