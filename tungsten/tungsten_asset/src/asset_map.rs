use std::collections::HashMap;

use super::{AssetId,AssetData};

pub struct AssetMap<T>{
    id: HashMap<AssetId,AssetData<T>>,
    name: HashMap<String,AssetId>,
}

impl<T> AssetMap<T>{
    pub fn new() -> Self{
        AssetMap{
            id: HashMap::new(),
            name: HashMap::new(),
        }
    }

    pub fn insert(&mut self,data: AssetData<T>){
        let id = data.id;
        self.name.insert(data.name.clone(),id);
        self.id.insert(id,data);
    }

    pub fn get(&self,id: AssetId) -> Option<&AssetData<T>>{
        self.id.get(&id)
    }

    pub fn get_mut(&mut self,id: AssetId) -> Option<&mut AssetData<T>>{
        self.id.get_mut(&id)
    }
    
    pub fn get_name(&self,name: &String) -> Option<&AssetData<T>>{
        if let Some(x) = self.name.get(name){
            self.id.get(x)
        }else{
            None
        }
    }

    pub fn get_mut_name(&mut self,name: &String) -> Option<&mut AssetData<T>>{
        if let Some(x) = self.name.get(name){
            self.id.get_mut(x)
        }else{
            None
        }
    }

    pub fn remove(&mut self,id: AssetId) -> Option<AssetData<T>>{
        let res = self.id.remove(&id);
        if let Some(ref x) = res{
            self.name.remove(&x.name);
        }
        res
    }

    pub fn remove_name(&mut self,name: &String) -> Option<AssetData<T>>{
        if let Some(x) = self.name.remove(name){
            self.id.remove(&x)
        }else{
            None
        }
    }

    pub fn contains_key(&self, id: AssetId) -> bool{
        self.id.contains_key(&id)
    }

    pub fn contains_key_name(&self, name: &String) -> bool{
        self.name.contains_key(name)
    }
}
