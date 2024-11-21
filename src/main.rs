#[macro_use]
extern crate glium;
use glium::{winit::event_loop::EventLoop, Surface};
use my_app::{ DrawEnv, MyApp};

mod my_app;
mod meshes;

fn main() {

    let init_draw:my_app::InitDraw<UsrEnv> = |event: &glium::winit::event_loop::EventLoop<()>,usr_env:&mut UsrEnv|{
            let mesh = &usr_env.mesh;
            let vertex_shader_src = std::fs::read_to_string("shaders/vertex.vert").unwrap();
            let fragment_shader_src = std::fs::read_to_string("shaders/fragment.frag").unwrap();
            let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
            .with_title("my app")
            .build(event);

            
            let vertex_buffer = glium::VertexBuffer::new(&display,mesh.into_vertex_slice() ).unwrap();
            let programs = glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None).unwrap();
            let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

            return DrawEnv{
                vertex_buffer,
                programs,
                indices,

                display,_window:window,
            };
    };

    let draw :my_app::UpdateDraw<UsrEnv> = |_event_loop: &glium::winit::event_loop::ActiveEventLoop,_usr_env: &mut UsrEnv,draw_env:&mut DrawEnv|{

        draw_env.vertex_buffer.map_write().set(0, /* vertex_buffer[0] */);

        let vertex_buffer = &draw_env.vertex_buffer;
        let indices = &draw_env.indices;
        let programs = &draw_env.programs;
        let display = &draw_env.display;

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        
        let uniforms = uniform! {
            screen_size: target.get_dimensions()
        };
        target.draw(vertex_buffer, indices, &programs, &uniforms,
            &Default::default()).unwrap();
        target.finish().unwrap();
              
    };

    let init: my_app::UsrInit<UsrEnv> = |_event,_usr_env,_app_env|{()};

    let update :my_app::UsrUpdate<UsrEnv> = |_event,usr_env,app_env|{
        usr_env.mesh.rotate_x(4.*app_env.dt);
        
    };
   
    struct UsrEnv {
        mesh : meshes::Mesh
    }

    let usr_env = UsrEnv{mesh :meshes::Mesh::new(meshes::TRIANGLE.to_vec())};

    
    //building event loop
    
    let event_loop = EventLoop::new().unwrap();
    let mut app: MyApp<UsrEnv> = MyApp::new(&event_loop,init_draw, draw, init, update, usr_env);
    
    event_loop.set_control_flow(glium::winit::event_loop::ControlFlow::Poll);
    event_loop.run_app(&mut app).unwrap();

             

}