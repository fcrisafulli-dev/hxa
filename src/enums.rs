use crate::meta::HXAMeta; //how are circular imports even allowed
use crate::node::HXAGeometryNode; //how are circular imports even allowed

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
pub enum HXANodeType {
    MetaOnly, // node only containing meta data.

    /// Node containing a geometry mesh, and meta data.
	Geometry(HXAGeometryNode),

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
                HXANodeType::Geometry(HXAGeometryNode::new())
            },
            2u8 => {
                HXANodeType::Image
            },
            _ => HXANodeType::Unknown
        }
    }
}

#[derive(Debug)]
pub enum  HXALayerDataType{
    UINT8 (Vec<u8>),
    INT32 (Vec<i32>),
    FLOAT (Vec<f32>),
    DOUBLE (Vec<f64>),
    Unknown
}

impl From<u8> for HXALayerDataType{
    fn from(value: u8) -> Self {
        match value {
            0u8 => {
                HXALayerDataType::UINT8(Vec::with_capacity(1))
            },
            1u8 => {
                HXALayerDataType::INT32(Vec::with_capacity(1))
            },
            2u8 => {
                HXALayerDataType::FLOAT(Vec::with_capacity(1))
            },
            3u8 => {
                HXALayerDataType::DOUBLE(Vec::with_capacity(1))
            },
            _ => HXALayerDataType::Unknown
        }
    }
}

