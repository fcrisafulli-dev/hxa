use crate::meta::HXAMeta;
use crate::node::{HXANode, HXAGeometryNode};
use crate::macros::{buffer,read_bytes};
use std::fs::File;
use std::io::{BufReader, Read};
use std::str;

#[derive(Debug)]
pub struct HXAFile{
    magic_number: u32, //The file begins with a file identifyer. it always has to be the 4 bytes "HxA", See definition of HAX_MAGIC_NUMBER. Since the magic number is always the same we dont store it in this structure even if it is always precent in files.
	pub version: u8,
    pub node_count: u32, // number of nodes in the file
    pub node_array: Vec<HXANode>
}


impl HXAFile {
    pub fn new() -> Self{
        HXAFile{
            magic_number: 0,
            version: 0,
            node_count: 0,
            node_array: Vec::with_capacity(1),
        }
    }

    pub fn read_header(self: &mut HXAFile, input: &mut BufReader<File>){
        //Read magic number
        self.magic_number = read_bytes!{input u32};
        
        //Read version number
        self.version = read_bytes!{input u8};

        //Read node count
        self.node_count = read_bytes!{input u32};

        for _ in 0..self.node_count{
            let mut new_node = HXANode::new();
            new_node.parse(input);
            self.node_array.push(new_node);
        }
    }

    /// Returns the first geometry node found
    /// 
    /// 
    /// The Some<> is a tuple with the metadata in `0` and the specific node in `1`
    /// 
    /// #### Deprecation
    /// The underlying structure of this library will likely be changed so that specific nodes such as `HXAGeometryNode` contain metadata.  
    /// Then, only the geometry node will be returned, simplifying the function
    /// 
    /// #### note:
    /// Currently there is no function to get multiple geometry nodes.    
    /// If a file has more than 1, they will need to be parsed manually.
    /// 
    /// 
    pub fn get_first_geometry(&self) -> Option<(&HXAGeometryNode, &Vec<HXAMeta>)> {
        for node in &self.node_array{
            match &node.node_type {
                crate::enums::HXANodeType::Geometry(gnode) => {
                    return Some((gnode,&node.meta_data));
                },
                _ => {},
            }
        }
        return None;
    }
}

impl From<&str> for HXAFile {
    fn from(value: &str) -> Self {
        let mut f = BufReader::new(
            File::open(value)
            .expect("Failed to open file")
        );

        let mut new_hxa_file = HXAFile::new();
        new_hxa_file.read_header(&mut f);

        new_hxa_file
    }
}
