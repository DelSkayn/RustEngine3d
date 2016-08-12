use std::path::PathBuf;

use std::str::Lines;

enum Error{
    ParseError(&'static str),
    Complete,
}

enum Symbol{
    Vertex{x: f32,y: f32,z: f32,w: Option<f32>},
    TextCoord{u: f32,v: f32, w: Option<f32>},
    Normal{x: f32,y: f32,z: f32},
    ParaSpace{u: f32,v: Option<f32>,w: Option<f32>},
    Face{vertex: Vec<u32>,texture: Option<Vec<u32>>,normal: Option<Vec<u32>>},
    MaterialFile{path: PathBuf},
}

struct ObjParser<'a>{
    data: Lines<'a>,
    error: Error,
}

impl ObjParser{
    fn new(data: &str) -> Self{
        ObjParser{
            data: data.lines(),
        }
    }

    fn parse_face<T: Iterator<Item = &str>>(&mut self,mut iter: T) -> Option<Symbol>{
        if let Some(x) = iter.next(){
            let faces = x.split("/");
            if let Some(x) = faces.next(){
                if let Ok(face) = x{
                    let res = Symbol::Face{vertex: 
                }else{
                    self.error = ParseError("Could not parse number").
                    None
                }
            }
        }else{
            self.error = ParseError("No symbol found.");
            None
        }
    }
}

impl Iterator for ObjParser{
    type Item = Symbol;

    fn next(&mut self) -> Option<Symbol>{
        while let Some(line) = data.next(){
            values = line.split_whitespace();
            if let Some(ty) = values.next(){
                match ty{
                    "f" => 
                    _ => continue,
                }
            }
        }
        None
    }

}
