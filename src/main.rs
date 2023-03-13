use hxa;

fn main() {
    println!("Opening file...");

    let my_hxa = hxa::HXAFile::from("sample_models/plugin.hxa");

    //dbg!(my_hxa);

    let model_geometry = my_hxa.get_first_geometry().expect("Expected to find a geometry node").0;

    use hxa::conventions::{hard,soft};

    let vertex_stack = &model_geometry.vertex_stack;

    let vertex_positions = vertex_stack
            .find(hard::BASE_VERTEX_LAYER_NAME)
            .expect("Expected to find a vertex layer")
            .as_vec_f32();

    let vertex_normals = vertex_stack
        .find(soft::LAYER_NORMALS)
        .expect("Expected to find a normal layer")
        .as_vec_f32();

    dbg!(vertex_positions);
    dbg!(vertex_normals);

    println!("Done");
}
