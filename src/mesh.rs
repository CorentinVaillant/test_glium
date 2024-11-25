use std::fs::read_to_string;

use glium::{implement_vertex, VertexBuffer};

#[derive(Clone, Copy,Debug)]
pub struct Vertex{
    pub position : [f32;4],
}implement_vertex!(Vertex,position);

impl Vertex {
    #[allow(dead_code)]
    fn new(x:f32,y:f32,z:f32,w:f32)->Self{
        Vertex{
            position :[x,y,z,w]
        }
    }

    fn transform(&mut self, trans_mat:[[f32;4];4]){
        for i in 0..4{
            let mut y = 0.;
            for k in 0..4{
                y+= trans_mat[i][k]*self.position[k];
            }
            self.position[i] = y;
        }
    }
}

impl From<[f32;3]> for Vertex {
    fn from(value: [f32;3]) -> Self {
        Vertex{
            position :[value[0],value[1],value[2],1.]
        }
    }
}

impl From<[f32;4]> for Vertex{
    fn from(value: [f32;4]) -> Self {
        Vertex{
            position :value
        }
    }
}

impl From<Vec<f32>> for Vertex{
    fn from(value: Vec<f32>) -> Self {
        let mut it = value.into_iter();
        let mut position = [0.;4];
        for i in 0..4{
            position[i] = it.next().unwrap_or(1.);   
        }

        Vertex{
            position
        }
    }
}

#[derive(Clone,Debug)]
#[allow(dead_code)]
pub struct Mesh{

    size : f32,
    position : [f32;3],

    vertecies :Vec<Vertex>,
    vertex_indices :Vec<usize>


}

impl From<Vec<Vertex>> for Mesh{
    fn from(vertecies: Vec<Vertex>) -> Self {
        Mesh{
            size :1.,
            position:[0.;3],

            vertecies,
            vertex_indices:vec![]
        }
    }
}


impl From<Vec<[f32;4]>> for Mesh{
    fn from(vec: Vec<[f32;4]>) -> Self {
        let mut result_vec = vec![];

        for tab in vec.into_iter(){
            result_vec.push(Vertex::from(tab));
        }

        Mesh::from(result_vec)

    }
}



impl Mesh {
    pub fn empty_mesh()->Mesh {
        Mesh{
            size : 1.,
            position :[0.;3],

            vertecies:vec![],
            vertex_indices:vec![]
        }
    }

    pub fn into_vertex_slice(&self)->&[Vertex]{
        self.vertecies.as_slice()
    }

    pub fn scale_applied(&mut self, scalar:f32){
        for vertex in self.vertecies.iter_mut(){
            for i in 0..4{
                vertex.position[i] = vertex.position[i] * scalar;

            }
        }
    }

    pub fn load_into_vertex_buffer(&self,buffer :& VertexBuffer<Vertex>){
        buffer.write(self.into_vertex_slice());
    }

    fn transform(&mut self, trans_mat :[[f32;4];4]){
        for vertex in self.vertecies.iter_mut(){
            vertex.transform(trans_mat);
        }
    }

    pub fn rotate_z(&mut self,theta:f32){
        let theta = theta % (2.*std::f32::consts::PI);
        let trans_mat = [
            [theta.cos(),-theta.sin(),0.,0.],
            [theta.sin(),theta.cos(),0.,0.],
            [0.,0.,1.,0.],
            [0.,0.,0.,1.]
        ];


        self.transform(trans_mat);
    }

    pub fn rotate_y(&mut self,theta:f32){
        let theta = theta % (2.*std::f32::consts::PI);
        let trans_mat = [
            [theta.cos(),0.,theta.sin(),0.],
            [0.,1.,0.,0.],
            [-theta.sin(),0.,theta.cos(),0.],
            [0.,0.,0.,1.]
        ];


        self.transform(trans_mat);
    }

}


impl Mesh{
    pub fn old_load_from_obj(path:&str)->Result<Mesh,std::io::Error>{
        let mut vertex_vec: Vec<[f32;4]> = vec![];


        let file = read_to_string(path)?;
        for line in file.lines(){
            match obj_parse_line_type(line) {

                ObjLineType::Vertex => {
                    let mut vertex_coord = [0.;4];
                    obj_parse_vertex(line, &mut vertex_coord);
                    vertex_vec.push(vertex_coord);

                },

                
                // ObjLineType::Comment=>println!("OBJ comment :{}",line),
                _=>()//TODO,
            };
        }

        
        Ok(Mesh::from(vertex_vec))
    }

    pub fn load_from_obj(path:&str)->Result<Mesh,std::io::Error>{//TODO

        let mut vertex_vec: Vec<[f32;4]> = vec![];


        let file = read_to_string(path)?;
        for line in file.lines(){
            match obj_parse_line_type(line) {

                ObjLineType::Vertex => {
                    let mut vertex_coord = [0.;4];
                    obj_parse_vertex(line, &mut vertex_coord);
                    vertex_vec.push(vertex_coord);

                },

                
                ObjLineType::Comment=>println!("OBJ comment :{}",line),
                _=>()//TODO,
            };
        }

        
        Ok(Mesh::from(vertex_vec))
    }

}

enum ObjLineType {
    Vertex,
    VertexNormal,
    TextureCoordinate,
    ParameterSpaceVertices,
    Line,
    Face,
    Comment,

    Empty,
    Unknow,
}

fn obj_parse_line_type(line:&str)->ObjLineType{
    let mut line = line.chars();

    match line.next(){
        Some(c) => match c {
            'v'=>match  line.next(){
                Some(c)=> match c {
                    ' '=> ObjLineType::Vertex,
                    't'=>ObjLineType::TextureCoordinate,
                    'n'=>ObjLineType::VertexNormal,
                    'p'=>ObjLineType::ParameterSpaceVertices,

                    _=>ObjLineType::Unknow
                }
                None => ObjLineType::Unknow
            },
            'f'=>ObjLineType::Face,
            'l'=>ObjLineType::Line,
            '#'=>ObjLineType::Comment,

            _=>ObjLineType::Unknow,
        },
        None => ObjLineType::Empty,
    }

}

fn obj_parse_vertex(line : &str,tab :&mut [f32;4]){
    let mut vec_float = parse_float(line).into_iter();
    tab.iter_mut().for_each(|x|{*x = vec_float.next().unwrap_or(1.)});
}

fn parse_float(line :&str)->Vec<f32>{
    let mut values = vec![];
    for val in line.split_whitespace(){
        //comment case
        if val.starts_with("#"){break;}

        //parse case
        match val.parse(){
            Ok(v)=> values.push(v),
            _=>()
        }
    }

    values
}