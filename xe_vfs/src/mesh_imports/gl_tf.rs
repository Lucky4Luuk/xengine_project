use xe_renderer::{
    Vertex,
    VertexIndex,
    VertexPosition,
    VertexNormal,
    VertexRGB
};

// use gltf::*;
use gltf::Semantic;

pub struct glTF {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<VertexIndex>
}

impl glTF {
    pub fn new(file_content: Vec<u8>) -> Result<Vec<glTF>, &'static str> {
        let (document, buffers, images) = match gltf::import_slice(file_content.as_slice()) {
            Ok((a,b,c)) => (a,b,c),
            Err(e) => {
                error!("{}", e);
                return Err("Failed to load glTF file!");
            }
        };

        //https://docs.rs/gltf/0.15.0/gltf/struct.Document.html

        let mut meshes = Vec::new();

        for mesh in document.meshes() {
            for primitive in mesh.primitives() {
                let mut vertices = Vec::new();
                let mut indices = Vec::new();

                let doc_positions = primitive.get(&Semantic::Positions).expect("Positions missing!");
                let doc_normals = primitive.get(&Semantic::Normals).expect("Normals missing!");
                let doc_colors = primitive.get(&Semantic::Colors(0)).expect("Colors missing!");

                meshes.push(Self {
                    vertices: vertices,
                    indices: indices,
                });
            }
        }

        Ok(meshes)
    }
}
