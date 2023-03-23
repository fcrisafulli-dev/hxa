use crate::enums::HXALayerDataType;
use crate::macros::{buffer,read_bytes,whereami,read_str};
use std::fs::File;
use std::io::{BufReader, Read, Seek};
use std::{str};


#[derive(Debug)]
pub struct  HXALayer {
    /// name of the layer. List of predefined names for common usages like uv, reference, blendshapes, weights ...
    pub name: String,

    /// 2 for uv, 3 for xyz or rgb, 4 for rgba;
    pub components: u8,

    /// Stored in the file as a uint8.
    pub layer_type: HXALayerDataType
}
impl HXALayer {
    fn new() -> Self {
        HXALayer {
            name: String::from("NO NAME"),
            components: 0,
            layer_type: HXALayerDataType::Unknown,
        }
    }

    fn parse(self: &mut HXALayer, input: &mut BufReader<File>, num_items: &u32) {
        // Get the name of the layer
        //whereami!(input);
        let name_length:u8 = read_bytes!(input u8);
        let mut name_buffer = buffer!(exactly name_length);
        let name = read_str!(input name_buffer);
        self.name = String::from(name);

        // Get the number of components for the layer
        self.components = read_bytes!(input u8);

        // Get layer type
        let u8_layer_type:u8 = read_bytes!(input u8);
        self.layer_type = HXALayerDataType::from(u8_layer_type);

        match &mut self.layer_type {
            HXALayerDataType::UINT8(uint_array) => {
                for _ in 0..((*num_items) * (self.components as u32)){
                    uint_array.push(read_bytes!(input u8));
                }
            },
            HXALayerDataType::INT32(int_array) => {
                for _ in 0..((*num_items) * (self.components as u32)){
                    int_array.push(read_bytes!(input i32));
                }
            },
            HXALayerDataType::FLOAT(float_array) => {
                for _ in 0..((*num_items) * (self.components as u32)){
                    float_array.push(read_bytes!(input f32));
                }
            },
            HXALayerDataType::DOUBLE(double_array) => {
                for _ in 0..((*num_items) * (self.components as u32)){
                    double_array.push(read_bytes!(input f64));
                }
            },
            HXALayerDataType::Unknown => {},
        }
    }

    pub fn try_as_vec_i32(&self) -> Option<&Vec<i32>>{
        match &self.layer_type{
            HXALayerDataType::INT32(int_array) => return Some(int_array),
            _ => return None,
        }
    }

    /// Gets and unwraps the underlying vector
    /// # Panics
    /// Panics if the underlying vector is the wrong type
    /// ## Recommendation
    /// Use this function if following a `HxA` standard where  a `HXALayer` with a specific name always has a specific type
    pub fn as_vec_i32(&self) -> &Vec<i32>{
        self.try_as_vec_i32().expect("Expected the underlying type to be a Vec<i32>")
    }

    pub fn try_as_vec_f32(&self) -> Option<&Vec<f32>>{
        match &self.layer_type{
            HXALayerDataType::FLOAT(float_array) => return Some(float_array),
            _ => return None,
        }
    }

    /// Gets and unwraps the underlying vector
    /// # Panics
    /// Panics if the underlying vector is the wrong type
    /// ## Recommendation
    /// Use this function if following a `HxA` standard where  a `HXALayer` with a specific name always has a specific type
    pub fn as_vec_f32(&self) -> &Vec<f32>{
        self.try_as_vec_f32().expect("Expected the underlying type to be a Vec<f32>")
    }

    /// Gets and unwraps the underlying vector and returns a copy of the vector in sets of 3
    /// # Panics
    ///  - If the underlying vector is the wrong type
    ///  - If the number of elements is not divisible by 3 
    /// ## Recommendation
    /// Use this function if following a `HxA` standard where  a `HXALayer` with a specific name always has a specific type
    /// 
    /// This was designed to make it easier to move data into `Vertex` objects from libraries such as `Vulkano` and `glium`
    pub fn as_tri_tup_vec_f32(&self) -> Vec<(f32,f32,f32)>{
        
        let vec_ref = self.as_vec_f32();
        if self.components != 3 {
            panic!("Components must be exactly 3")
        }

        let num_tris = vec_ref.len()/3;
        let mut out:Vec<(f32,f32,f32)> = Vec::with_capacity(num_tris);

        for multiplier in 0..num_tris{
            let idx = multiplier * (self.components as usize);

            out.push((
                *vec_ref.get(idx).unwrap(),
                *vec_ref.get(idx).unwrap(),
                *vec_ref.get(idx).unwrap(),
            ));
        }

        out
    }

    pub fn try_as_vec_f64(&self) -> Option<&Vec<f64>>{
        match &self.layer_type{
            HXALayerDataType::DOUBLE(double_array) => return Some(double_array),
            _ => return None,
        }
    }

    /// Gets and unwraps the underlying vector
    /// # Panics
    /// Panics if the underlying vector is the wrong type
    /// ## Recommendation
    /// Use this function if following a `HxA` standard where  a `HXALayer` with a specific name always has a specific type
    pub fn as_vec_f64(&self) -> &Vec<f64>{
        self.try_as_vec_f64().expect("Expected the underlying type to be a Vec<f64>")
    }

    pub fn try_as_vec_u8(&self) -> Option<&Vec<u8>>{
        match &self.layer_type{
            HXALayerDataType::UINT8(uint_array) => return Some(uint_array),
            _ => return None,
        }
    }

    /// Gets and unwraps the underlying vector
    /// # Panics
    /// Panics if the underlying vector is the wrong type
    /// ## Recommendation
    /// Use this function if following a `HxA` standard where  a `HXALayer` with a specific name always has a specific type
    pub fn as_vec_u8(&self) -> &Vec<u8>{
        self.try_as_vec_u8().expect("Expected the underlying type to be a Vec<u8>")
    }
}

#[derive(Debug)]
pub struct HXALayerStack{
    pub layer_count: u32,
    pub layers: Vec<HXALayer>
}

impl HXALayerStack {
    pub fn new() -> Self {
        HXALayerStack { 
            layer_count: 0,
            layers: Vec::with_capacity(1)
        }
    }

    pub fn parse(self: &mut HXALayerStack, input: &mut BufReader<File>, num_items: &u32){

        self.layer_count = read_bytes!(input u32);

        for _ in 0..self.layer_count{
            let mut new_layer = HXALayer::new();
            new_layer.parse(input, num_items);
            self.layers.push(new_layer)
        }
    }

    /// Finds the first layer with the specified name
    pub fn find(&self, layer_name:&str) -> Option<&HXALayer>{
        // In the future I might change layer names to just be string slices too
        let search_string = String::from(layer_name);

        for layer in &self.layers{
            if layer.name == search_string{
                return Some(layer);
            }
        }
        None
    }
}

