use super::mesh;

pub struct ObjLoader{
    src: String,
}

#[derive(Debug)]
pub enum ObjError{
    InvalidFormat,
}

#[derive(Eq,PartialEq)]
enum IndexFormat{
    Normal,
    V_T_N,
    V_Null_N,
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
    
    pub fn load(self) -> Result<mesh::Mesh<u16>,ObjError>{
        let mut vertecies_found = 0;
        let mut normals_found = 0;
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
                    vertecies_found  += 1;
                    let mut value = [0.0; 3];

                    let vertex_word = try!(iter.next().ok_or(ObjError::InvalidFormat));
                    value[0] = try!(vertex_word.parse::<f32>().or(Err(ObjError::InvalidFormat)));

                    let vertex_word = try!(iter.next().ok_or(ObjError::InvalidFormat));
                    value[1] = try!(vertex_word.parse::<f32>().or(Err(ObjError::InvalidFormat)));

                    let vertex_word = try!(iter.next().ok_or(ObjError::InvalidFormat));
                    value[2] = try!(vertex_word.parse::<f32>().or(Err(ObjError::InvalidFormat)));

                    vertecies.push(value);
                }
                "vn" => {
                    normals_found  += 1;
                    let mut value = [0.0; 3];

                    let normal_word = try!(iter.next().ok_or(ObjError::InvalidFormat));
                    value[0] = try!(normal_word.parse::<f32>().or(Err(ObjError::InvalidFormat)));

                    let normal_word = try!(iter.next().ok_or(ObjError::InvalidFormat));
                    value[1] = try!(normal_word.parse::<f32>().or(Err(ObjError::InvalidFormat)));

                    let normal_word = try!(iter.next().ok_or(ObjError::InvalidFormat));
                    value[2] = try!(normal_word.parse::<f32>().or(Err(ObjError::InvalidFormat)));

                    normals.push(value);
                }
                "f" => {
                    if format == IndexFormat::NotTested{
                        let test_word = try!(iter.peek().ok_or(ObjError::InvalidFormat)).clone();
                        let format_split: Vec<&str> = test_word.split("\\").collect();
                        if format_split.len() < 3 || format_split[2] == ""{
                            format = IndexFormat::Normal;
                        }else{
                            if format_split[1] == "" {
                                format = IndexFormat::V_Null_N;
                            }else{
                                format = IndexFormat::V_T_N;
                            }
                        }
                    }
                    match format{
                        IndexFormat::Normal => {
                            for _ in 0..3{
                                let index_word = try!(iter.next().ok_or(ObjError::InvalidFormat));
                                let index_w: String = index_word.chars().filter(|a|{ a != &'\\' }).collect();//remove posible trailing /
                                let value = try!(index_w.parse::<u16>().or(Err(ObjError::InvalidFormat)));
                                index.push(value);
                            }
                        }
                        IndexFormat::V_T_N => {
                            for _ in 0..3{
                                let index_word = try!(iter.next().ok_or(ObjError::InvalidFormat));
                                let index_w:Vec<_> = index_word.split('\\').collect();
                                if index_w.len() < 3 { return Err(ObjError::InvalidFormat); }
                                index.push(try!(index_w[0].parse::<u16>().or(Err(ObjError::InvalidFormat))));
                                index_texture.push(try!(index_w[1].parse::<u16>().or(Err(ObjError::InvalidFormat))));
                                index_normal.push(try!(index_w[2].parse::<u16>().or(Err(ObjError::InvalidFormat))));
                            }
                        }
                        IndexFormat::V_Null_N => {
                            for _ in 0..3{
                                let index_word = try!(iter.next().ok_or(ObjError::InvalidFormat));
                                let index_w: Vec<_> = index_word.split("\\").collect();
                                if index_w.len() < 3 { return Err(ObjError::InvalidFormat); }
                                index.push(try!(index_w[0].parse::<u16>().or(Err(ObjError::InvalidFormat))));
                                index_normal.push(try!(index_w[2].parse::<u16>().or(Err(ObjError::InvalidFormat))));
                            }
                        }
                        _ => unreachable!()
                    }
                }
                _ => {
                    warn!("Found unsupported obj symbol");
                }
            }
        }
        info!("Found vertecies: {}, normals: {}, indecies: {}",vertecies.len(),normals.len(),0);
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
