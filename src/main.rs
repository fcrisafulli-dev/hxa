use hxa;

fn main() {
    println!("Opening file...");

    let my_hxa = hxa::HXAFile::from("sample_models/plugin.hxa");

    //dbg!(my_hxa);

    let model_geometry = my_hxa.get_first_geometry().expect("Expected to find a geometry node").1;

    dbg!(model_geometry);

    let vert_vec = model_geometry.vertex_stack
        .find(hxa::conventions::hard::BASE_VERTEX_LAYER_NAME).expect("Expected to find a vertex layer")
        .as_vec_f32();

    

    dbg!(vert_vec);
    // let file_location = input.seek(std::io::SeekFrom::Current (0)).expect("Could not get current position!");
    // println!("{:X?}",file_location);
    // dbg!(file_location);

    println!("Done");
}
