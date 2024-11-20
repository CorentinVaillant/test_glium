#[macro_use]
extern crate glium;
use glium::{winit::event_loop::EventLoop, Surface};
use meshes::Mesh;

mod my_app;
mod meshes;

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("my app")
        .build(&event_loop);


    let draw = |_event: &glium::winit::event_loop::ActiveEventLoop,app_attr: &mut my_app::AppAttr|{

        match &app_attr.shader_program {
            Some(program) => {
                let mut target = display.draw();
                target.clear_color(0.0, 0.0, 1.0, 1.0);
        
                
                let uniforms = uniform! {
                    screen_size: target.get_dimensions()
                };
                target.draw(&app_attr.vertex_buffer, app_attr.indices, program, &uniforms,
                    &Default::default()).unwrap();
                target.finish().unwrap();
            },
            None => todo!(),
        };

        
    };

    let init = |_event,app_attr: &mut my_app::AppAttr|{
        let mesh = Mesh::new(meshes::TRIANGLE.to_vec());
        let vertex_buffer = glium::VertexBuffer::new(&display,mesh.into_vertex_slice() ).unwrap();
    
        let vertex_shader_src = std::fs::read_to_string("shaders/vertex.vert").unwrap();
    
    
        let fragment_shader_src = std::fs::read_to_string("shaders/fragment.frag").unwrap();
    
        let program = glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None).unwrap();

        app_attr.shader_program = Some(program);
        app_attr.vertex_buffer = vertex_buffer;
    
    };

    let update = |_event: &glium::winit::event_loop::ActiveEventLoop,_delta,_app_attr: &mut my_app::AppAttr|{
        ()
    };
   
    
    let mut app = my_app::MyApp::new(draw,init,update,&display);

    event_loop.set_control_flow(glium::winit::event_loop::ControlFlow::Poll);
    event_loop.run_app(&mut app).unwrap();

             

}