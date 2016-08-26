use super::StaticMeshNoTexture;
use super::glium::backend::Context;
use super::super::RenderObjects;
use super::super::Transform;
use std::rc::Rc;

pub struct CacheMetaData(StaticMeshNoTexture);

pub struct Cache{
    loaded_meshes: Vec<StaticMeshNoTexture>,
    transforms: Vec<Transform>,
}

impl Cache{
    pub fn new() -> Self{
        Cache{
            loaded_meshes: Vec::new(),
            transforms: Vec::new(),
        }
    }

    pub fn load(&mut self,context: &Rc<Context>,que: &RenderObjects){
        let len = que.len();
        self.fetch(context,que,len);
        self.cache(que,len);
    }

    fn cache(&mut self,que: &RenderObjects,len: usize){
        for i in 0..len{
            if que[i].changed(){
                let data = que[i].get();
                self.transforms[i] = data.transform;
            }
        }
    }

    fn fetch(&mut self,context: &Rc<Context>,que: &RenderObjects,len: usize){
        if len > self.loaded_meshes.len(){
            for i in self.loaded_meshes.len()..len{
                let data = que[i].get();
                let loaded_mesh = data.mesh.use_data(|mesh|{
                    StaticMeshNoTexture::from_mesh(context.clone(),mesh).unwrap()
                });
                self.loaded_meshes.push(loaded_mesh);
                println!("{:?}",data.transform);
                self.transforms.push(data.transform);
            }
        }
    }

    pub fn mesh(&self,i: usize) -> &StaticMeshNoTexture{
        &self.loaded_meshes[i]
    }

    pub fn transform(&self,i: usize) -> &Transform{
        &self.transforms[i]
    }
}
