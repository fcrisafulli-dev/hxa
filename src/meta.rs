use crate::enums::HXAMetaDataType;
use crate::macros::{buffer,read_bytes,whereami,read_str};
use std::fs::File;
use std::io::{BufReader, Read, Seek};
use std::str;

#[derive(Debug)]
pub struct HXAMeta{
    pub name: String,
    pub meta_type: HXAMetaDataType,
    pub data_length: u32
}

impl HXAMeta {
    pub fn new() -> Self {
        HXAMeta { 
            name: String::from("MISSING NAME"),
            meta_type: HXAMetaDataType::Unknown,
            data_length: 0,
            
        }
    }

    pub fn parse(self: &mut HXAMeta, input: &mut BufReader<File>){

        // Length of the name in bytes
        let name_length:u8 = read_bytes!(input u8);
        
        let mut name_buffer = buffer!(exactly name_length);
        let data_name = read_str!(input name_buffer);

        self.name = String::from(data_name);

        self.meta_type = HXAMetaDataType::from(read_bytes!(input u8));
        self.data_length = read_bytes!(input u32);

        match &mut self.meta_type {
            HXAMetaDataType::INT64 { int_array } => {
                for _ in 0..self.data_length{
                    let int64_read = read_bytes!(input i64);
                    int_array.push(int64_read);
                }
            }
            HXAMetaDataType::DOUBLE { double_array } => {
                for _ in 0..self.data_length{
                    let double_read = read_bytes!(input f64);
                    double_array.push(double_read);
                }
            },

            HXAMetaDataType::TEXT { text } => {
                let mut meta_text_buffer = buffer!(exactly self.data_length);
                let slice = read_str!(input meta_text_buffer);
                *text = String::from(slice);
            },

            HXAMetaDataType::META { meta_array } => {
                //We expect self.data_length more meta datas

                for _ in 0..self.data_length{
                    let mut new_metadata = HXAMeta::new();
                    new_metadata.parse(input);
                    meta_array.push(new_metadata);
                }
            },
            _ => {}
        }
    }
}