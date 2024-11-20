
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
pub const TRIANGLE :VertexArr<4> = [
    Vertex { position: [ 0.0 *100., 0.5 *100.,0.] },
    Vertex { position: [-0.43*100.,-0.25*100.,0.] },
    Vertex { position: [ 0.43*100.,-0.25*100.,0.] },
    Vertex { position: [ 0.43*100., 0.25*100.,0.] },
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
}