extern crate cgmath;
use self::cgmath::*;

use super::glium::backend::Context;

use super::super::RenderObjects;
use super::StaticMeshNoTexture;
use super::super::tungsten_asset::AssetId;

use std::rc::Rc;
use std::collections::HashMap;

pub struct CachedData{
    pub transform: Decomposed<Vector3<f32>,Quaternion<f32>>,
    pub mesh: Rc<StaticMeshNoTexture>,
}

pub type CachedRenderQue = Vec<CachedData>;

pub struct Cache{
    loaded_meshes: HashMap<AssetId,Rc<StaticMeshNoTexture>>,
    que: CachedRenderQue,
}

impl Cache{
    pub fn new() -> Self{
        Cache{
            loaded_meshes: HashMap::new(),
            que: CachedRenderQue::new(),
        }
    }

    pub fn load(&mut self,context: &Rc<Context>,que: &RenderObjects){
        let len = que.len();
        self.fetch(context,que,len);
        self.cache(que,len);
    }

    fn cache(&mut self,que: &RenderObjects,len: usize){
        for i in 0..len{
            if que[i].object.changed(){
                let data = que[i].object.get();
                self.que[i].transform = data.transform;
            }
        }
    }

    fn fetch(&mut self,context: &Rc<Context>,new: &RenderObjects,len: usize){
        // Cache new render objects
        for i in self.que.len()..len{
            let data = new[i].object.get();

            // test if the mesh is already present.
            if let Some(x) = self.loaded_meshes.get(new[i].mesh().id()){
                self.que.push(CachedData{
                    transform: data.transform,
                    mesh: x.clone(),
                });
                continue;
            }

            // if not load.
            let loaded_mesh = new[i].mesh().data().use_data(|mesh|{
                StaticMeshNoTexture::from_mesh(context.clone(),mesh).unwrap()
            });
            let mesh = Rc::new(loaded_mesh);
            self.loaded_meshes.insert(new[i].mesh().id().clone(),mesh.clone());
            self.que.push(CachedData{
                transform: data.transform,
                mesh: mesh,
            });
        }
    }

    pub fn que(&self) -> &CachedRenderQue{
        &self.que
    }
}
