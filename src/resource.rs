use std::collections::HashMap;

trait Resource{
    fn get_size() -> usize;
}

trait ResourceLoader{
    type Output: Resource;
    type LoadData;

    fn load(LoadData) -> Self::Output;
}

struct ResourceHandeler{
    resource_map: HashMap<String,Rc<Resource>>,
}

struct ResourceHandle<T: Resource>{
    res: T,
}
