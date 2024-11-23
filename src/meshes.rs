use glium::VertexBuffer;


type VertexArr<const N:usize> = [Vertex;N];

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
}

impl Vertex{
    fn matrix_mul(&mut self,m :[[f32;4];4]){
        for i in 0..3{
            self.position[i] = self.position.iter().zip(m[i]).map(|(x,a)|{a*x}).sum::<f32>() + m[i][3];
        }
    }



    fn scale(&mut self, s:f32){
        for i in 0..3{
            self.position[i] *= s;
        }
    }
}
implement_vertex!(Vertex, position);

#[allow(unused)]
pub const CUBE :VertexArr<8> = [
    Vertex { position: [ 100.0, -100.0, -100.0] },
    Vertex { position: [ 100.0,  100.0, -100.0] },
    Vertex { position: [ 100.0, -100.0,  100.0] },
    Vertex { position: [ 100.0,  100.0,  100.0] },
    Vertex { position: [-100.0, -100.0, -100.0] },
    Vertex { position: [-100.0,  100.0, -100.0] },
    Vertex { position: [-100.0, -100.0,  100.0] },
    Vertex { position: [-100.0,  100.0,  100.0] },
];

#[allow(unused)]
pub const TRIANGLE :VertexArr<3> = [
    Vertex { position: [ 0.0 *100., 0.5 *100.,0.] },
    Vertex { position: [-0.43*100.,-0.25*100.,0.] },
    Vertex { position: [ 0.43*100.,-0.25*100.,0.] },

];

pub struct Mesh{
    vertecies :Vec<Vertex>
}

impl Mesh {
    pub fn new(vertecies :Vec<Vertex>)->Self{
        Mesh{vertecies}
    }

    pub fn into_vertex_slice(&self)->&[Vertex]{
        self.vertecies.as_slice()
    }

    pub fn write_into_buffer(&self,buffer:&mut VertexBuffer<Vertex>){
        buffer.write(self.into_vertex_slice());
    }

    #[allow(dead_code)]
    fn matrix_mul(&mut self, m :[[f32;4];4]){
        self.vertecies.iter_mut().for_each(|vertex|{
            vertex.matrix_mul(m);
        });
    }

    #[allow(dead_code)]
    pub fn rotate_x (&mut self,teta:f32){
        self.matrix_mul(r_x(teta));
    }

    pub fn rotate_y(&mut self, teta:f32){
        self.matrix_mul(r_y(teta));
    }

    #[allow(dead_code)]
    pub fn scale(&mut self, s:f32){
        self.vertecies.iter_mut().for_each(|vertex|{
            vertex.scale(s);
        });
    }
}

fn r_x(teta:f32)->[[f32;4];4]{
    [
        [1.,0.,0.,0.],
        [0.,teta.cos(),-teta.sin(),0.],
        [0.,teta.sin(),-teta.cos(),0.],
        [0.,0.,0.,0.],
    ]
}

fn r_y(teta:f32)->[[f32;4];4]{
    [
        [teta.cos(), 0., teta.sin(),0.],
        [0.,1.,0.,0.],
        [-teta.sin(),0.,teta.cos(),0.],
        [0.;4]
    ]

}