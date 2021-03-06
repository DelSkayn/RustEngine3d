
use super::{Assets,Mesh,Container};

pub struct ObjLoader;

struct ObjObject{
    pub name: String,
    pub vertecies: Vec<[f32; 3]>,
    pub tex_coords: Vec<[f32; 2]>,
    pub normals: Vec<[f32; 3]>,
    pub faces: Vec<Vec<u32>>,
    pub normal_faces: Option<Vec<Vec<u32>>>,
    pub texture_faces: Option<Vec<Vec<u32>>>,
}

struct Vertecies{
    tex_coords: Option<Vec<[f32; 2]>>,
    normals: Vec<[f32; 3]>,
}

struct Indecies{
    vertex: Vec<u32>,
    normals: Vec<u32>,
    texture: Option<Vec<u32>>,
}

impl ObjLoader{
    pub fn load(file: Vec<u8>,place: Container<Mesh>){
        if let Ok(x) = String::from_utf8(file){
            let mut object = ObjObject{
                name: String::new(),
                vertecies: Vec::new(),
                tex_coords: Vec::new(),
                normals: Vec::new(),
                faces: Vec::new(),
                normal_faces: None,
                texture_faces: None,
            };
            trace!("Obj: parsing file");
            for line in x.lines(){
                if !Self::parse_line(line,&mut object){
                    return;
                }
            }

            if object.normals.len() == 0{
                warn!("Did not found normals, Object must have normals to be loaded");
                return;
            }

            if object.normal_faces.is_some()
                && object.normal_faces.as_ref().unwrap().len() != object.faces.len(){
                    warn!("Amount of texture indecies or amount of normal indecies to small.");
                    return;
                }

            if object.texture_faces.is_some()
                && object.texture_faces.as_ref().unwrap().len() != object.faces.len(){
                    warn!("Amount of texture indecies or amount of normal indecies to small.");
                    return;
                }

            place.change(Self::to_mesh(object));

        }else{
            warn!("Could not confert file into utf8");
            return;
        }


        info!("Done loading");
    }

    fn parse_line(line: &str,object: &mut ObjObject) -> bool{
        let mut white = line.split_whitespace();
        if let Some(identifier) = white.next(){
            match identifier{
                "#" => return true,//comment, ignore
                "f" => return Self::parse_face(white,object),
                "v" => return Self::parse_vertex(white,object),
                "vn" => return Self::parse_normal(white,object),
                "vt" => return Self::parse_text_coord(white,object),
                "o" => return Self::parse_name(white,object),
                _ => {
                    //warn!("Unkown identifier \"{}\" in obj file: TODO make error once properly implemented",x);
                    return true;
                },
            }
        }else{
            // blank lines are allowed
            return true;
        }
    }

    fn parse_name<'a,I: Iterator<Item = &'a str>>(mut line: I,object: &mut ObjObject) -> bool{
        let name = if let Some(v) = line.next(){v} else {warn!("missing name");return false;};
        object.name = String::from(name);
        return true;
    }

    fn parse_vertex<'a,I: Iterator<Item = &'a str>>(mut line: I,object: &mut ObjObject) -> bool{
        let x_str = if let Some(v) = line.next(){v} else {warn!("missing vertex");return false;};
        let y_str = if let Some(v) = line.next(){v} else {warn!("missing vertex");return false;};
        let z_str = if let Some(v) = line.next(){v} else {warn!("missing vertex");return false;};

        if let Some(_) = line.next(){
            warn!("to many vertecies on a single line");
            return false;
        }

        let x: f32 = if let Ok(v) = x_str.parse(){v} else {warn!("could not parse vertex");return false;};
        let y: f32 = if let Ok(v) = y_str.parse(){v} else {warn!("could not parse vertex");return false;};
        let z: f32 = if let Ok(v) = z_str.parse(){v} else {warn!("could not parse vertex");return false;};

        object.vertecies.push([x,y,z]);
        return true;
    }

    fn parse_normal<'a,I: Iterator<Item = &'a str>>(mut line: I,object: &mut ObjObject) -> bool{
        let x_str = if let Some(v) = line.next(){v} else {warn!("missing normals");return false;};
        let y_str = if let Some(v) = line.next(){v} else {warn!("missing normals");return false;};
        let z_str = if let Some(v) = line.next(){v} else {warn!("missing normals");return false;};

        if let Some(_) = line.next(){
            warn!("to many normals on a single line");
            return false;
        }// to ma

        let x: f32 = if let Ok(v) = x_str.parse(){v} else {warn!("could not parse normals");return false;};
        let y: f32 = if let Ok(v) = y_str.parse(){v} else {warn!("could not parse normals");return false;};
        let z: f32 = if let Ok(v) = z_str.parse(){v} else {warn!("could not parse normals");return false;};

        object.normals.push([x,y,z]);
        return true;
    }

    fn parse_text_coord<'a,I: Iterator<Item = &'a str>>(mut line: I,object: &mut ObjObject) -> bool{
        let u_str = if let Some(v) = line.next(){v} else {warn!("missing texture coord");return false;};
        let v_str = if let Some(v) = line.next(){v} else {warn!("missing texture coord");return false;};

        let u: f32 = if let Ok(v) = u_str.parse(){v} else {warn!("could not parse texture coord");return false;};
        let v: f32 = if let Ok(v) = v_str.parse(){v} else {warn!("could not parse texture coord");return false;};

        object.tex_coords.push([u,v]);
        return true;
    }

    fn parse_face<'a,I: Iterator<Item = &'a str>>(line: I,object: &mut ObjObject) -> bool{
        let mut face = Vec::new();
        let mut face_text = None;
        let mut face_normal = None;

        let mut amount = 0;
        for face_part in line{
            amount += 1;
            let mut index = face_part.split("/");
            let v_index_str = if let Some(v) = index.next(){v} else {warn!("missing index!");return false;};
            let t_index_str = if let Some(v) = index.next(){v} else {warn!("missing index!");return false;};
            let n_index_str = if let Some(v) = index.next(){v} else {warn!("missing index!");return false;};

            let v_index: u32 = if let Ok(v) = v_index_str.parse(){v} else {warn!("could not parse vertex face");return false;};
            let t_index: Result<u32,_> = t_index_str.parse();
            let n_index: Result<u32,_> = n_index_str.parse();

            face.push(v_index);

            // Adding texture and normal indecies if they exist.
            if let Ok(x) = t_index{
                if face.len() > 1{
                    warn!("missing texture coord face indecies in obj file");
                    return false;
                }
                if let None = face_text{
                    face_text = Some(Vec::new());
                }
                face_text.as_mut().unwrap().push(x);
            }

            if let Ok(x) = n_index{
                if face.len() > 1{
                    warn!("missing normal face indecies in obj file");
                    return false;
                }
                if let None = face_normal{
                    face_normal= Some(Vec::new());
                }
                face_normal.as_mut().unwrap().push(x);
            }
        }

        // Faces of with les then 3 indecies are lines or dots not faces
        if amount < 3{
            warn!("Missing indecies for a face.");
            return false;
        }

        // push face to there place
        object.faces.push(face);

        if let Some(x) = face_text{
            if let None = object.texture_faces{
                object.texture_faces= Some(Vec::new());
            }
            object.texture_faces.as_mut().unwrap().push(x);
        }

        if let Some(x) = face_normal{
            if let None = object.normal_faces{
                object.normal_faces = Some(Vec::new());
            }
            object.normal_faces.as_mut().unwrap().push(x);
        }

        return true;
    }

    fn to_mesh(mut object: ObjObject) -> Mesh{
        let mut index = Self::triangulate(&mut object);

        Self::process_indecies(&mut index);

        let vertecies = Self::unwrap_indecies(&index,&object);

        Mesh{
            vertecies: object.vertecies,
            normals: vertecies.normals,
            texture_coords: vertecies.tex_coords,
            indecies: index.vertex,
            material: Assets::get_material(&"default".to_string()).data
        }
    }

    /// Remove faces with more than 3 vertecies.
    fn triangulate(object:&ObjObject) -> Indecies{
        trace!("Obj: triangulating");
        let mut vertex = Vec::with_capacity(object.faces.len() * 3);
        let mut texture = None;
        let mut normals = Vec::with_capacity(object.normal_faces.as_ref().unwrap().len() * 3);

        for ref face in &object.faces{
            // todo create a better algoritm.
            // this does not work for a lot of faces.
            let start = face[0];
            let mut current = face[1];
            for i in &face[2..]{
                vertex.push(start);
                vertex.push(current);
                vertex.push(i.clone());
                current = i.clone();
            }
        }

        // do the same traingulation for texture coords if they exist and normals.
        if let Some(ref x) = object.texture_faces{
            let mut res = Vec::with_capacity(object.texture_faces.as_ref().unwrap().len() * 3);
            for face in x{
                let start = face[0];
                let mut current = face[1];
                for i in &face[2..]{
                    res.push(start);
                    res.push(current);
                    res.push(i.clone());
                    current = i.clone();
                }
            }
            texture = Some(res);
        }

        for face in object.normal_faces.as_ref().unwrap(){
            let start = face[0];
            let mut current = face[1];
            for i in &face[2..]{
                normals.push(start);
                normals.push(current);
                normals.push(i.clone());
                current = i.clone();
            }
        }

        Indecies{
            vertex: vertex,
            normals: normals,
            texture: texture,
        }

    }

    /// Subtrace 1 from all indecies.
    /// Obj format is 1 indexed so we need to confert it.
    fn process_indecies(indecies: &mut Indecies){
        trace!("Obj: processing indecies");
        for i in 0..indecies.vertex.len(){
            indecies.vertex[i] -= 1;
        }

        for i in 0..indecies.normals.len(){
            indecies.normals[i] -= 1;
        }

        if let Some(ref mut x) = indecies.texture.as_mut(){
            for i in 0..x.len(){
                x[i] -= 1;
            }
        }

    }

    /// Make mesh adressable by a single index.
    ///
    /// TODO: Current implementation removes flat shading. Fix that.
    /// Atleast I think it does. Might not though. But it probebly does.
    /// TODO: Test wether this actually removes flat shading.
    fn unwrap_indecies(indecies: &Indecies,object: &ObjObject) -> Vertecies{
        trace!("Obj: unwrap indecies");
        let length = indecies.vertex.len();

        let mut normals = Vec::with_capacity(length);
        // Lot faster then manualy pushing everything first.
        // Would love a function which allows pushing a lot of objects in a single function call if
        // the type supports Copy for that sweet memcpy perf.
        unsafe{normals.set_len(length)};

        let borrow = &indecies.normals;
        for i in 0..length{
            normals[indecies.vertex[i] as usize] = object.normals[borrow[i] as usize];
        }


        let tex_coords;
        if indecies.texture.is_some(){
            let mut res = Vec::with_capacity(length);
            // Lot faster then manualy pushing everything first.
            unsafe{res.set_len(length)};

            let borrow = indecies.texture.as_ref().unwrap();

            for i in 0..length{
                res[indecies.vertex[i] as usize] = object.tex_coords[borrow[i] as usize];
            }

            tex_coords = Some(res);
        }else{
            tex_coords = None;
        }

        Vertecies{
            normals: normals,
            tex_coords: tex_coords,
        }
    }
}


