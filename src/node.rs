
use crate::layer::HXALayerStack;
use crate::meta::HXAMeta;
use crate::enums::HXANodeType;
use crate::macros::{buffer,read_bytes,whereami,read_str};
use std::fs::File;
use std::io::{BufReader, Read, Seek};

#[derive(Debug)]
pub struct HXANode {
    pub node_type: HXANodeType, //u8, //just 'type', in original spec
    pub metadata_count: u32,
    pub meta_data: Vec<HXAMeta>,
}

impl HXANode{
    pub fn new() -> Self{
        HXANode{
            node_type: HXANodeType::Unknown,
            metadata_count: 0,
            meta_data: Vec::with_capacity(1)
        }
    }

    pub fn parse(self: &mut HXANode, input: &mut BufReader<File>){

        //Read node type
        self.node_type = HXANodeType::from(read_bytes!(input u8)); 

        
        //Read metadata count
        self.metadata_count = read_bytes!(input u32);

        //Get metadata
        for _ in 0 .. self.metadata_count{
            let mut meta_data = HXAMeta::new();
            meta_data.parse(input);
            self.meta_data.push(meta_data);
            // println!("name length:{} {:?} {:?}",name_length, name_buffer, s);
        }


        //Extract data based on node type
        match &mut self.node_type {
            HXANodeType::MetaOnly => (),
            HXANodeType::Geometry(node) => {
                node.parse(input);
            },
            HXANodeType::Image => (),
            HXANodeType::Unknown => panic!("Unknown node type"),
        }
    }

}

#[derive(Debug)]
pub struct HXAGeometryNode{
    /// number of vertices
    pub vertex_count: u32, 

    /// stack of vertex arrays. the first layer is always the vertex positions
    pub vertex_stack:HXALayerStack,

    /// number of corners
    pub edge_corner_count:u32,

    /// stack of corner arrays, the first layer is allways a reference array (see below)
    pub corner_stack:HXALayerStack,

    /// stack of edge arrays
    pub edge_stack:HXALayerStack, 

    /// number of polygons
    pub face_count:u32, 

    /// stack of per polygon data.
    pub face_stack:HXALayerStack,
}

impl HXAGeometryNode {
    pub fn new() -> Self{
        HXAGeometryNode{
            vertex_count: 0,
            vertex_stack: HXALayerStack::new(),
            edge_corner_count: 0,
            corner_stack: HXALayerStack::new(),
            edge_stack: HXALayerStack::new(),
            face_count: 0,
            face_stack: HXALayerStack::new(),
        }
    }

    pub fn parse(self: &mut HXAGeometryNode, input: &mut BufReader<File>){
        self.vertex_count = read_bytes!(input u32);
        self.vertex_stack.parse(input, &self.vertex_count);


        self.edge_corner_count = read_bytes!(input u32);
        self.corner_stack.parse(input, &self.edge_corner_count);
        self.edge_stack.parse(input, &self.edge_corner_count);


        self.face_count = read_bytes!(input u32);
        self.face_stack.parse(input, &self.face_count);
    }
}