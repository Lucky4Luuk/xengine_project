use std::collections::HashMap;
use wavefront_obj::obj::{self, TVertex};

use photic::{
    Vertex,
    VertexIndex,
    VertexPosition,
    VertexNormal,
    VertexRGB,
    VertexUV,
};

pub struct Obj {
    // pub nfaces: usize,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<VertexIndex>
}

impl Obj {
    pub fn new(file_content: String) -> Result<Vec<Obj>, &'static str> {
        debug!("Parsing obj file...");
        let obj_set = match obj::parse(file_content) {
            Ok(set) => set,
            // Err(why) => return Err(&format!("Couldn't parse mesh: {:?}", why))
            Err(_why) => return Err("Couldn't parse mesh!")
        };
        let objects = obj_set.objects;
        // if objects.len() != 1 {
        //     return Err("Currently only Obj files with a single model are supported!")
        // }

        // let object = objects.into_iter().next().expect("Failed to get the first object!");
        // if object.geometry.len() != 1 {
        //     debug!("Geometries: {}", object.geometry.len());
        //     return Err("Currently only Obj files with a single piece of geometry are supported!")
        // }

        // let geometry = object.geometry.into_iter().next().unwrap();
        let mut meshes: Vec<Obj> = Vec::new();

        for object in objects {
            debug!("Loading {}", object.name);
            debug!("{} vertices", object.vertices.len());
            //Build up a hashmap based on vertex id, so we can filter out any duplicate vertices
            let mut vertex_cache: HashMap<obj::VTNIndex, VertexIndex> = HashMap::new();
            let mut vertices: Vec<Vertex> = Vec::new();
            let mut indices: Vec<VertexIndex> = Vec::new();
            // let mut nfaces = 0;

            for geometry in object.geometry {
                debug!("{} shapes", geometry.shapes.len());

                for shape in geometry.shapes {
                    if let obj::Primitive::Triangle(a, b, c) = shape.primitive {
                        for key in &[a, b, c] {
                            if let Some(vertex_index) = vertex_cache.get(key) {
                                indices.push(*vertex_index);
                            } else {
                                let p = object.vertices[key.0];
                                //TODO: Generate normals and uv coordinates for meshes that are missing them.
                                let n = object.normals[key.2.ok_or("Missing normal for a vertex!")?];
                                // let tv = object.tex_vertices[key.1.ok_or("Missing UV for a vertex!")?];
                                let tv = if let Some(idx) = key.1 {
                                    object.tex_vertices[idx]
                                } else {
                                    TVertex {
                                        u: 0.0,
                                        v: 0.0,
                                        w: 0.0,
                                    }
                                };
                                let position = VertexPosition::new([p.x as f32, p.y as f32, p.z as f32]);
                                let color = VertexRGB::new([255, 255, 255]);
                                let normal = VertexNormal::new([n.x as f32, n.y as f32, n.z as f32]);
                                let uv = VertexUV::new([tv.u as f32, tv.v as f32, tv.w as f32]);
                                let vertex = Vertex {
                                    position: position,
                                    color: color,
                                    normal: normal,
                                    uv: uv
                                };
                                let vertex_index = vertices.len() as VertexIndex;

                                vertex_cache.insert(*key, vertex_index);
                                vertices.push(vertex);
                                indices.push(vertex_index);
                            }
                        }
                        // nfaces += 1;
                    } else {
                        return Err("Currently only Obj files with only triangles are supported!");
                    }
                }
            }

            meshes.push(Obj {
                // nfaces: nfaces,
                vertices: vertices,
                indices: indices
            });
        }

        Ok(meshes)

        // Ok(Obj {
        //     vertices: vertices,
        //     indices: indices,
        // })
    }
}
