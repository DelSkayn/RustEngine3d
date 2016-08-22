use std::collections::HashMap;

use asset::metadata::MetaData;
use asset::Container;

use super::{StaticMeshNoTexture,StaticMeshNoTextureQue,RenderQue,Context};

pub struct CacheMetaData(StaticMeshNoTexture);

impl MetaData for CacheMetaData{}

pub struct Cache{
    registered: Vec<Container<Mesh>>,
}

impl Cache{
    pub fn new() -> Self{
        Cache{
            registered: Vec::new(),
        }
    }

    pub fn load(&mut self,context: Arc<Context>,mesh: &Container<Mesh>){
        let loaded_mesh = mesh.use_data(|mesh|{
             Box::new(StaticMeshNoTexture::from_mesh(context,mesh).unwrap());
        });
        self.registered.push(mesh.clone());
        context.meta.change(CacheMetaData(loaded_mesh));
    }

    pub fn process(&mut self,context: Arc<Context>,que: RenderQue) -> StaticMeshNoTextureQue{
        let res = Vec::with_capacity(que.len());
        for obj in que{
            if que.
        }
        res
    }
}
