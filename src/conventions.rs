/// 
pub mod hard {
    /// Despite being called 'vertex' this layer contains just the vertex positions of a mesh   
    /// ### Note
    /// Other mesh features such as normals can be obtained from the soft conventions
    /// 
    pub const BASE_VERTEX_LAYER_NAME: &str = "vertex";
    pub const VERTEX_LAYER_ID: u32 = 0;
    pub const VERTEX_LAYER_COMPONENTS: u32 = 3;

    pub const CORNER_LAYER_NAME: &str = "reference";
    pub const CORNER_LAYER_ID: u32 = 0;
    pub const CORNER_LAYER_COMPONENTS: u32 = 1;
    //const CORNER_LAYER_TYPE HXA_LDT_INT32
    
    pub const NEIGHBOUR_LAYER_NAME: &str = "neighbour";
    //const NEIGHBOUR_LAYER_TYPE HXA_LDT_INT32
}

/// Soft conventions which may not be present in a file
pub mod soft {
    pub const LAYER_SEQUENCE0         : &str = "sequence";
    pub const LAYER_NAME_UV0          : &str = "uv";
    pub const LAYER_NORMALS           : &str = "normal";
    pub const LAYER_TANGENT           : &str = "tangent";
    pub const LAYER_CREASES           : &str = "creases";
    pub const LAYER_SELECTION         : &str = "selection";
    pub const LAYER_SKIN_WEIGHT       : &str = "skining_weight";
    pub const LAYER_SKIN_REFERENCE    : &str = "skining_reference";
    pub const LAYER_BLENDSHAPE        : &str = "blendshape";
    pub const LAYER_ADD_BLENDSHAPE    : &str = "addblendshape";
    pub const LAYER_MATERIAL_ID       : &str = "material";
}