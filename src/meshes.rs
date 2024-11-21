
type VertexArr<const N:usize> = [Vertex;N];

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
}
implement_vertex!(Vertex, position);

#[allow(unused)]
pub const CUBE :VertexArr<8> = [
    Vertex { position: [ 100., 100., 100.] },
    Vertex { position: [ 100., 100.,-100.] },
    Vertex { position: [ 100.,-100., 100.] },
    Vertex { position: [ 100.,-100.,-100.] },
    Vertex { position: [-100., 100., 100.] },
    Vertex { position: [-100., 100.,-100.] },
    Vertex { position: [-100.,-100., 100.] },
    Vertex { position: [-100.,-100.,-100.] },
];

#[allow(unused)]
pub const TRIANGLE :VertexArr<3> = [
    Vertex { position: [ 0.0 *100., 0.5 *100.,0.] },
    Vertex { position: [-0.43*100.,-0.25*100.,0.] },
    Vertex { position: [ 0.43*100.,-0.25*100.,0.] },
    // Vertex { position: [ 0.43*100., 0.25*100.,0.] },
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

    #[allow(dead_code)]
    fn matrix_mul(&mut self, m :[[f32;4];4]){
        self.vertecies.iter_mut().for_each(|vertex|{
            for i in 0..3{
                vertex.position[i] = vertex.position.iter().zip(m[i]).map(|(x,a)|{a*x}).sum::<f32>() + m[i][3];
            }
        });
    }

    #[allow(dead_code)]
    pub fn rotate_x (&mut self,teta:f32){
        self.matrix_mul(r_x(teta));
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