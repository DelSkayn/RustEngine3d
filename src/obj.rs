use super::render::mesh;

pub struct ObjLoader{
    src: String,
}

#[derive(Debug)]
pub enum ObjError{
    InvalidFormat(&'static str),
} 
#[derive(Eq,PartialEq,Debug)]
enum IndexFormat{
    Normal,
    VTN,
    VNullN,
    NotTested,
}
//this temp
//might not work well with multiple textures.
impl ObjLoader{
    pub fn new(src: String) -> Self{
        ObjLoader{
            src: src,
        }
    }
    
    pub fn load(self) -> Result<mesh::Mesh,ObjError>{
        let mut format = IndexFormat::NotTested;
        let mut vertecies = Vec::new();
        let mut normals = Vec::new();
        let mut index = Vec::<u16>::new();
        let mut index_texture = Vec::new();
        let mut index_normal = Vec::new();

        let mut iter = self.src.split_whitespace().peekable();
        while let Some(word) = iter.next(){
            match word{
                "v" => {
                    trace!("Found vertex");
                    let mut value = [0.0; 3];

                    let vertex_word = try!(iter.next().ok_or(ObjError::InvalidFormat("Missing a vertex after V")));
                    value[0] = try!(vertex_word.parse::<f32>().or(Err(ObjError::InvalidFormat("Could not parse vertex"))));

                    let vertex_word = try!(iter.next().ok_or(ObjError::InvalidFormat("Missing a vertex after V")));
                    value[1] = try!(vertex_word.parse::<f32>().or(Err(ObjError::InvalidFormat("Could not parse vertex"))));

                    let vertex_word = try!(iter.next().ok_or(ObjError::InvalidFormat("Missing a vertex after V")));
                    value[2] = try!(vertex_word.parse::<f32>().or(Err(ObjError::InvalidFormat("Could not parse vertex"))));

                    vertecies.push(value);
                }
                "vn" => {
                    trace!("Found normal");
                    let mut value = [0.0; 3];

                    let normal_word = try!(iter.next().ok_or(ObjError::InvalidFormat("Missing a normal after V")));
                    value[0] = try!(normal_word.parse::<f32>().or(Err(ObjError::InvalidFormat("Could not parse normal"))));

                    let normal_word = try!(iter.next().ok_or(ObjError::InvalidFormat("Missing a normal after V")));
                    value[1] = try!(normal_word.parse::<f32>().or(Err(ObjError::InvalidFormat("Could not parse normal"))));

                    let normal_word = try!(iter.next().ok_or(ObjError::InvalidFormat("Missing a normal after V")));
                    value[2] = try!(normal_word.parse::<f32>().or(Err(ObjError::InvalidFormat("Could not parse normal"))));

                    normals.push(value);

                }
                "f" => {
                    trace!("Found index");
                    if format == IndexFormat::NotTested{
                        let test_word = try!(iter.peek().ok_or(ObjError::InvalidFormat("could not get index"))).clone();
                        let format_split: Vec<&str> = test_word.split("/").collect();
                        debug!("Split found {} words", format_split.len());
                        if format_split.len() < 3 || format_split[2] == ""{
                            format = IndexFormat::Normal;
                        }else{
                            if format_split[1] == "" {
                                format = IndexFormat::VNullN;
                            }else{
                                format = IndexFormat::VTN;
                            }
                        }
                        trace!("Found index format {:?}",format);
                    }
                    match format{
                        IndexFormat::Normal => {
                            for _ in 0..3{
                                let index_word = try!(iter.next().ok_or(ObjError::InvalidFormat("Could not get index")));
                                let index_w: String = index_word.chars().filter(|a|{ a != &'/' }).collect();//remove posible trailing /
                                let value = try!(index_w.parse::<u16>().or(Err(ObjError::InvalidFormat("Could not parse index"))));
                                index.push(value-1);
                            }
                        }
                        IndexFormat::VTN => {
                            for _ in 0..3{
                                let index_word = try!(iter.next().ok_or(ObjError::InvalidFormat("Could not get index")));
                                let index_w:Vec<_> = index_word.split('/').collect();
                                if index_w.len() < 3 { return Err(ObjError::InvalidFormat("Index missing index value")); }
                                index.push(try!(index_w[0].parse::<u16>().or(Err(ObjError::InvalidFormat("Could not parse index"))))-1);
                                index_texture.push(try!(index_w[1].parse::<u16>().or(Err(ObjError::InvalidFormat("Could not parse index"))))-1);
                                index_normal.push(try!(index_w[2].parse::<u16>().or(Err(ObjError::InvalidFormat("Could not parse index"))))-1);
                            }
                        }
                        IndexFormat::VNullN => {
                            for _ in 0..3{
                                let index_word = try!(iter.next().ok_or(ObjError::InvalidFormat("Could not get index")));
                                let index_w: Vec<_> = index_word.split('/').collect();
                                if index_w.len() < 3 { return Err(ObjError::InvalidFormat("Could not parse index")); }
                                index.push(try!(index_w[0].parse::<u16>().or(Err(ObjError::InvalidFormat("Could not parse index"))))-1);
                                index_normal.push(try!(index_w[2].parse::<u16>().or(Err(ObjError::InvalidFormat("Could not parse index"))))-1);
                            }
                        }
                        _ => unreachable!()
                    }
                }
                _ => {
                    debug!("Found unsupported obj symbol");
                }
            }
        }
        info!("Found vertecies: {}, normals: {}, indecies: {}",vertecies.len(),normals.len(),0);

        //place normals in proper place
        if format == IndexFormat::VTN || format == IndexFormat::VNullN {
            let old = normals;
            normals = Vec::with_capacity(vertecies.len());
            unsafe{
                //faster than pushing vertecies.len() times
                normals.set_len(vertecies.len());
            }
            for i in 0..vertecies.len(){
                normals[index[i] as usize] = old[index_normal[i] as usize];
            }
        }

        /*
        if format == IndexFormat::VTN {
            let mut old = normals;
            normals = Vec::with_capacity(vertecies.len());
            unsafe{
                normals.set_len(vertecies.len());
            }
            for i in 0..vertecies.len(){
                normals[index[i] as usize] = old[index_normal[i]];
            }
        }
        */

        let mut vertex_res = Vec::with_capacity(vertecies.len());
        for i in 0..vertecies.len(){
            vertex_res.push(mesh::MeshVertex{
                position: vertecies[i],
                normal: normals[i],
            });
        }
        Ok(mesh::Mesh{
            vertecies: vertex_res,
            index: index,
        })
    }
}

#[cfg(test)]
mod test{
    #![allow(unused_import)]
    #![allow(unused_attributes)]
    #![allow(unused_attributes)]
    #![allow(unused_variables)]

    use std::io;
    use std::fs::File;
    use std::io::BufReader;
    use std::io::Read;
    use super::*;
    
    #[test]
    fn test(){
        let file = File::open("res/teapot.obj").unwrap();
        let mut reader = BufReader::new(file);
        let mut src = String::new(); 
        reader.read_to_string(&mut src).unwrap();

        let loader = ObjLoader::new(src);

        let mesh =  loader.load().unwrap();
    }
}
