

use super::{Mesh,Container};

pub struct ObjLoader;

pub struct ObjObject{
    pub name: String,
    pub vertices: Vec<[f32; 3]>,
    pub tex_coords: Vec<[f32; 2]>,
    pub normals: Vec<[f32; 3]>,
    pub faces: Vec<Vec<f32>>,
    pub normal_faces: Option<Vec<Vec<f32>>>,
    pub texture_faces: Option<Vec<Vec<f32>>>,
}


impl ObjLoader{
    pub fn load(file: Vec<u8>,place: Container<Mesh>){
        if let Ok(x) = String::from_utf8(file){
            let mut object = ObjObject{
                name: String::new(),
                vertices: Vec::new(),
                tex_coords: Vec::new(),
                normals: Vec::new(),
                faces: Vec::new(),
                normal_faces: None,
                texture_faces: None,
            };
            for line in x.lines(){
                if !Self::parse_line(line,&mut object){
                    return;
                }
            }
        }else{
            warn!("Could not confert file into utf8");
        }
    }

    fn parse_line(line: &str,object: &mut ObjObject) -> bool{
        let mut white = line.split_whitespace();
        if let Some(identifier) = white.next(){
            match identifier{
                "#" => return true,//comment, ignore
                "f" => return Self::parse_face(white,object),
                "v" => return Self::parse_vertex(white,object),
                x => {
                    warn!("Unkown identifier \"{}\" in obj file: TODO make error once properly implemented",x); 
                    return true;
                },
            }
        }else{
            // blank lines are allowed
            return true;
        }
    }

    fn parse_vertex<'a,I: Iterator<Item = &'a str>>(mut line: I,object: &mut ObjObject) -> bool{
        let x_str = if let Some(v) = line.next(){v} else {warn!("missing vertex");return false;};
        let y_str = if let Some(v) = line.next(){v} else {warn!("missing vertex");return false;};
        let z_str = if let Some(v) = line.next(){v} else {warn!("missing vertex");return false;};

        if let Some(_) = line.next(){
            warn!("to many vertecies on a single line");
            return false;
        }// to ma

        let x: f32 = if let Ok(v) = x_str.parse(){v} else {warn!("could not parse vertex");return false;};
        let y: f32 = if let Ok(v) = y_str.parse(){v} else {warn!("could not parse vertex");return false;};
        let z: f32 = if let Ok(v) = z_str.parse(){v} else {warn!("could not parse vertex");return false;};

        object.vertices.push([x,y,z]);
        return true;
    }

    fn parse_face<'a,I: Iterator<Item = &'a str>>(mut line: I,object: &mut ObjObject) -> bool{
        return true;
    }
}


