use std::collections::HashMap;


/*
 * The trait each rescource must implement
 */
trait Resource{
    fn get_size() -> usize;
    fn get_data() -> &[u8];
}

/*
 * The trait an resource loader must implement.
 */
trait ResourceLoader{
    type Output: Resource;
    type LoadData;

    fn load(LoadData) -> Self::Output;
}

struct ResourceStruct<T>
    where T:Resource{
    count: usize,
    size: usize,
    res: Resource,
}

/*
 * The resource Manager manages the lifetime, loading and referencing
 * of the resource
 *
 * Each resource is identified by a unique name, it is not recomended to use the 
 * file path since a file might contain multiple resources. 
 *
 * TODO: Figure out hashers in rust
 */
pub struct ResourceManager{
    resource_map: HashMap<String,Rc<Option<Resource>>>,
}

/*
 * The handle for using a resources. it should not hold a direct reference
 * since a handle is directly returned without it neccesary being loaded.
 */

impl ResourceManager{
}
