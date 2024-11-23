use std::{fs::File, io::{BufRead, BufReader}};

use crate::meshes::{Mesh, Vertex, CUBE};

pub fn load_wavefront(path: &str)-> Mesh{

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    
    let mut vertecies: Vec<Vertex> = vec![];

    for r_line in reader.lines(){
        match r_line {
            Err(e)=>println!("{e}"),
            Ok(line_str)=>{

                let mut line = line_str.chars();
                let mut opt_char = line.next();
                while opt_char != None {
                    let char = opt_char.unwrap();
                    if char == '#'{
                        line.last();
                    }



                    
                    opt_char = line.next();
                }
            }
        }
    } 
    

    return Mesh::new(CUBE.to_vec());
}