use photic::{
    Vertex,
    VertexIndex,
};

use luminance::context::GraphicsContext;

#[macro_use] extern crate log;

use std::error::Error;
use std::fs::File;
use std::io::Read as _;
use std::path::Path;

pub mod mesh_imports;
use mesh_imports::{
    obj::Obj,
    // gl_tf::glTF,
};

mod resource_storage;
pub use resource_storage::ResourceStorage;

pub enum FileType<T> {
    Other(T),
    TextFile,
    ObjFile,
    GlTfFile, //WIP
    GlTfBinFile, //Not supported right now
    ShaderFile,
    MaterialFile,
    LuaFile,

    Image
}

pub struct VirtualFile {
    pub path: String,
    pub filename: String,
    pub filetype: FileType<String>,
}

impl VirtualFile {
    pub fn new(path: String) -> Result<VirtualFile, &'static str> {
        let path_object = Path::new(&path);
        // debug!("Attempting to open {}", path_object.display());

        if path_object.is_file() == false {
            error!("Failed to load file `{}`!", path);
            return Err("Path doesn't lead to a file!");
        }

        //We can safely unwrap this, as we know for sure the path points to a path
        let filename = match path_object.file_name().unwrap().to_str() {
            Some(result) => result,
            None => {
                error!("Filename contains invalid characters!");
                return Err("Filename contains invalid characters!");
            }
        };

        let filetype = match path_object.extension() {
            None => FileType::Other("Unknown".to_string()),
            Some(extension) => {
                match extension.to_str() {
                    Some(result) => {
                        match result {
                            //Models
                            "obj" => FileType::ObjFile,
                            "gltf" => FileType::GlTfFile,
                            "glb" => FileType::GlTfBinFile,

                            //Code
                            "glsl" => FileType::ShaderFile,
                            "lua" => FileType::LuaFile,

                            //Textures
                            "png" => FileType::Image,
                            "jpeg" => FileType::Image,
                            "jpg" => FileType::Image,
                            "bmp" => FileType::Image,

                            //Other
                            "txt" => FileType::TextFile,
                            _ => FileType::Other(result.to_string())
                        }
                    },
                    None => FileType::Other("Unknown".to_string())
                }
            }
        };

        Ok(VirtualFile {
            path: path.clone(),
            filename: filename.to_string(),
            filetype: filetype,
        })
    }

    pub fn read(&self) -> Result<String, &'static str> {
        let path = Path::new(&self.path);

        info!("Attempting to read file...");

        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(why) => {
                error!("{}", why.description());
                return Err("Failed to open file!");
            }
        };
        let mut content = String::new();
        if let Err(why) = file.read_to_string(&mut content) {
            error!("{}", why.description());
            // return Err(why.description());
            return Err("Failed to read file!");
        }

        info!("Done!");

        Ok(content)
    }

    //TODO: Implement this
    pub fn read_bytes(&self) -> Result<(Vec<u8>, usize), &'static str> {
        let path = Path::new(&self.path);

        info!("Attempting to read file...");

        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(why) => {
                error!("{}", why.description());
                return Err("Failed to open file!");
            }
        };
        let mut content = Vec::new();
        let size = match file.read_to_end(&mut content) {
            Ok(size) => size,
            Err(why) => {
                error!("{}", why.description());
                // return Err(why.description());
                return Err("Failed to read file!");
            }
        };

        info!("Done!");

        Ok((content, size))
    }

    pub fn get_mesh(&self) -> Result<Vec<(Vec<Vertex>, Vec<VertexIndex>)>, &'static str>
    {
        match self.filetype {
            FileType::ObjFile => {
                let file_content = match self.read() {
                    Ok(content) => content,
                    Err(why) => return Err(why)
                };

                let objs = match Obj::new(file_content) {
                    Ok(objs) => objs,
                    Err(err) => return Err(err)
                };

                let mut meshes: Vec<(Vec<Vertex>, Vec<VertexIndex>)> = Vec::new();

                for obj in objs {
                    meshes.push((obj.vertices, obj.indices));
                }

                Ok(meshes)
            },
            _ => return Err("Cannot parse file as mesh!")
        }
    }

    pub fn get_texture(&self) -> Result<image::RgbaImage, &'static str> {//into_rgba
        let img = match self.filetype {
            FileType::Image => {
                let (content, _size) = match self.read_bytes() {
                    Ok((content, size)) => (content, size),
                    Err(why) => return Err(why)
                };

                match image::load_from_memory(&content[..]) {
                    Ok(img) => Ok(img.into_rgba()),
                    Err(err) => {
                        error!("Error loading image: {}", err);
                        Err("Failed to load image!")
                    }
                }
            },
            _ => {
                error!("Not a texture!");
                Err("Not a texture!")
            }
        };

        Ok(img.expect("Image file failed to load"))
    }
}
