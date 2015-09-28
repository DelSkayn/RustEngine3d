use std::collections::HashMap;

trait Resource{
    fn get_size() -> usize;
}

struct ResourceHandeler{
    resource_map: HashMap<String,Rc<Resource>>,
}

struct ResourceHandle<T: Resource>{
    res: T,
}
