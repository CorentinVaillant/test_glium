#[macro_use]
extern crate glium;
use glium::{winit::event_loop::EventLoop, Surface};

mod my_app;

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .build(&event_loop);

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 3],
    }
    implement_vertex!(Vertex, position);
    let cube = vec![
        Vertex { position: [ 1./4., 1./4., 1./4.] },
        Vertex { position: [ 1./4., 1./4.,-1./4.] },
        Vertex { position: [ 1./4.,-1./4., 1./4.] },
        Vertex { position: [ 1./4.,-1./4.,-1./4.] },
        Vertex { position: [-1./4., 1./4., 1./4.] },
        Vertex { position: [-1./4., 1./4.,-1./4.] },
        Vertex { position: [-1./4.,-1./4., 1./4.] },
        Vertex { position: [-1./4.,-1./4.,-1./4.] },
    ];

    let _triangle = vec![
        Vertex { position: [-0.5, -0.5 ,0.] },
        Vertex { position: [ 0.0,  0.5 ,0.] },
        Vertex { position: [ 0.5, -0.25,0.] }
    ];

    let vertex_buffer = glium::VertexBuffer::new(&display, &cube).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

    let vertex_shader_src = std::fs::read_to_string("shaders/vertex.vert").unwrap();


    let fragment_shader_src = std::fs::read_to_string("shaders/fragment.frag").unwrap();

    let program = glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None).unwrap();
   
    
    let mut app = my_app::MyApp::new(|_event|{
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
            &Default::default()).unwrap();
        target.finish().unwrap();
        
    });

    event_loop.set_control_flow(glium::winit::event_loop::ControlFlow::Poll);
    event_loop.run_app(&mut app).unwrap();

             

}