
use std::fs::File;
use std::io::{BufReader, Read, Seek};
use std::str;

macro_rules! whereami {
    ($reader:tt) => {
        {
            let file_location = $reader.seek(std::io::SeekFrom::Current(0)).expect("Could not get current position!");
            println!("I am here:  {:X?}  !",file_location)
        }
    };
}

/// creates a buffer with the size of the specified type
macro_rules!  buffer{
    ($typ:ty) => {
        [0u8; std::mem::size_of::<$typ>()]
    };

    (exactly $size:expr) => {
        vec![0u8; $size as usize]
    }
}

/// Reads enough bytes to produce the desired type
/// # Example
/// ```rust
/// let foo:u32 = read_bytes!(input u32);
/// ```
macro_rules!  read_bytes{
    ($r:tt $typ:tt) => {
        {
            let mut buffer = buffer!($typ);
            let result = $r.read_exact(&mut buffer);
            match result {
                Ok(_) => {
                    // whereami!($r);
                    // println!("^^ Location, OK read type:{}",stringify!($typ))
                },
                Err(_) => {
                    println!("read_type: {}",stringify!($typ));
                    panic!("Failed read_bytes")
                },
            }
            $typ::from_le_bytes(buffer)
        }
    }
}

/// Reads enough bytes to produce an str
macro_rules!  read_str{
    ($reader:ident $buffer:tt) => {
        {
            let result = $reader.read_exact(&mut $buffer);
            match result {
                Ok(_) => (),
                Err(_) => {
                    println!("read_type: {}",stringify!($buffer));
                    panic!("Expected to read more bytes")
                },
            }
            str::from_utf8(&$buffer).expect("Expected a valid utf8 format")
        }
    }
}

#[derive(Debug)]
pub enum HXANodeType {
    MetaOnly, // node only containing meta data.

    /// Node containing a geometry mesh, and meta data.
	Geometry{

            /// number of vertices
            vertex_count: u32, 

            /// stack of vertex arrays. the first layer is always the vertex positions
			vertex_stack:HXALayerStack,

            /// number of corners
			edge_corner_count:u32,

            /// stack of corner arrays, the first layer is allways a reference array (see below)
			corner_stack:HXALayerStack,

            /// stack of edge arrays
			edge_stack:HXALayerStack, 

            /// number of polygons
			face_count:u32, 

            /// stack of per polygon data.
			face_stack:HXALayerStack, 
    },

    /// node containing a 1D, 2D, 3D, or Cube image, and meta data.
	Image, 

    /// If the file designates the node type as anything other than `Meta`, `Geomatry` or`Image`
	Unknown, 
}

impl From<u8> for HXANodeType{
    fn from(value: u8) -> Self {
        match value {
            0u8 => {
                HXANodeType::MetaOnly
            },
            1u8 => {
                HXANodeType::Geometry{
                    vertex_count: 0,
                    vertex_stack: HXALayerStack::new(),
                    edge_corner_count: 0,
                    corner_stack: HXALayerStack::new(),
                    edge_stack: HXALayerStack::new(),
                    face_count: 0,
                    face_stack: HXALayerStack::new(),
                }
            },
            2u8 => {
                HXANodeType::Image
            },
            _ => HXANodeType::Unknown
        }
    }
}

#[derive(Debug)]
pub enum HXAMetaDataType{
    /// # Assumption
    /// `data_length` is the number of ints to read
    INT64 {
        int_array: Vec<i64>
    },

    /// # Assumption
    /// `data_length` is the number of doubles to read
    DOUBLE {
        double_array: Vec<f64>
    },


	NODE,

    /// # Assumption
    /// `data_length` is the number of characters in the text
	TEXT {
        text: String
    },


	BINARY,

    /// # Assumption
    /// Contains more meta data.
    /// `data_length` is the number of meta data entries
	META {
        meta_array: Vec<HXAMeta>
    },

	COUNT,
    Unknown
}

impl From<u8> for HXAMetaDataType{
    fn from(value: u8) -> Self {
        match value {
            0u8 => {
                HXAMetaDataType::INT64 {
                    int_array: Vec::with_capacity(1)
                }
            },
            1u8 => {
                HXAMetaDataType::DOUBLE {
                    double_array: Vec::with_capacity(3)
                }
            },
            2u8 => {
                HXAMetaDataType::NODE
            },
            3u8 => {
                HXAMetaDataType::TEXT {
                    text:String::from("NO TEXT")
                }
            },
            4u8 => {
                HXAMetaDataType::BINARY
            },
            5u8 => {
                HXAMetaDataType::META{
                    meta_array: Vec::with_capacity(1)
                }
            },
            6u8 => {
                HXAMetaDataType::COUNT
            },
            _ => HXAMetaDataType::Unknown
        }
    }
}

#[derive(Debug)]
pub struct HXAMeta{
    pub name: String,
    pub meta_type: HXAMetaDataType,
    pub data_length: u32
}

impl HXAMeta {
    fn new() -> Self {
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


#[derive(Debug)]
pub enum  HXALayerDataType{
    UINT8 {uint_array: Vec<u8>},
    INT32 {int_array: Vec<i32>},
    FLOAT {float_array: Vec<f32>},
    DOUBLE {double_array: Vec<f64>},
    Unknown
}

impl From<u8> for HXALayerDataType{
    fn from(value: u8) -> Self {
        match value {
            0u8 => {
                HXALayerDataType::UINT8{
                    uint_array: Vec::with_capacity(1)
                }
            },
            1u8 => {
                HXALayerDataType::INT32{
                    int_array: Vec::with_capacity(1)
                }
            },
            2u8 => {
                HXALayerDataType::FLOAT{
                    float_array: Vec::with_capacity(1)
                }
            },
            3u8 => {
                HXALayerDataType::DOUBLE{
                    double_array: Vec::with_capacity(1)
                }
            },
            _ => HXALayerDataType::Unknown
        }
    }
}

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
            HXALayerDataType::UINT8 { uint_array } => {
                for _ in 0..*num_items{
                    uint_array.push(read_bytes!(input u8));
                }
            },
            HXALayerDataType::INT32 { int_array } => {
                for _ in 0..*num_items{
                    int_array.push(read_bytes!(input i32));
                }
            },
            HXALayerDataType::FLOAT { float_array } => {
                for _ in 0..((*num_items) * (self.components as u32)){
                    float_array.push(read_bytes!(input f32));
                }
            },
            HXALayerDataType::DOUBLE { double_array } => {
                for _ in 0..*num_items{
                    double_array.push(read_bytes!(input f64));
                }
            },
            HXALayerDataType::Unknown => {},
        }
    }
}

#[derive(Debug)]
pub struct HXALayerStack{
    pub layer_count: u32,
    pub layers: Vec<HXALayer>
}

impl HXALayerStack {
    fn new() -> Self {
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
}

#[derive(Debug)]
pub struct HXAFile{
    magic_number: u32, //The file begins with a file identifyer. it always has to be the 4 bytes "HxA", See definition of HAX_MAGIC_NUMBER. Since the magic number is always the same we dont store it in this structure even if it is always precent in files.
	pub version: u8,
    pub node_count: u32, // number of nodes in the file
    pub node_array: Vec<HXANode>
}

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
            HXANodeType::Geometry { vertex_count, vertex_stack, edge_corner_count, corner_stack, edge_stack, face_count, face_stack } => {
                *vertex_count = read_bytes!(input u32);
                vertex_stack.parse(input, vertex_count);


                *edge_corner_count = read_bytes!(input u32);
                corner_stack.parse(input, edge_corner_count);
                edge_stack.parse(input, edge_corner_count);
                

                *face_count = read_bytes!(input u32);
                face_stack.parse(input, &face_count);

                //following this data will be 3 u32, seems like they contain face info. 
                //Im assuming the HxA exporter im using is not complete yet
            },
            HXANodeType::Image => (),
            HXANodeType::Unknown => panic!("Unknown node type"),
        }
    }

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
