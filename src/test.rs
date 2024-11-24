use crate::mesh;

pub fn test_mesh(){
    let mesh = mesh::Mesh::load_from_obj("./obj/cube.obj").unwrap();
}