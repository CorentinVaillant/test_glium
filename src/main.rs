

use my_glium_util::app::{App, ApplicationContext};
use glium::{index::NoIndices, uniform, winit::event_loop::EventLoop, Surface};
use my_glium_util::mesh::mesh::{Mesh, Vertex};


const OBJ_PATH : &str = "./obj/triangle.obj";

fn main() {
    println!("test mesh");

    println!("Starting app");
    let mut app: App<MyApp> = App::new("my app");


    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(glium::winit::event_loop::ControlFlow::Poll);
    event_loop.run_app(&mut app).unwrap();

}

struct DrawContext{
    vertex_buffer: glium::VertexBuffer<Vertex>,
    program: glium::Program,
    indices:NoIndices
}


struct MyApp {
    mesh :Mesh,

    draw_context : DrawContext,

    pub time: std::time::Instant,
    pub dt  :f32
}

impl ApplicationContext for MyApp {
    fn new(display: &glium::Display<glium::glutin::surface::WindowSurface>)->Self {
        let mut mesh = Mesh::old_load_from_obj(OBJ_PATH).expect("mesh could not be load");
        mesh.scale_applied(100.);
        // let mesh = Mesh::from(primitives_mesh::TRIANGLE.to_vec());
        

        let vertex_shader_src = std::fs::read_to_string("shaders/vertex.vert").unwrap();
        let fragment_shader_src = std::fs::read_to_string("shaders/fragment.frag").unwrap();

        let vertex_buffer: glium::VertexBuffer<Vertex> = glium::VertexBuffer::dynamic(display, mesh.into_vertex_slice()).unwrap();

        let program= glium::Program::from_source(display, &vertex_shader_src, &fragment_shader_src, None).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        MyApp { 
            mesh, 
            draw_context: DrawContext{
                vertex_buffer,
                program,
                indices
            },

            time : std::time::Instant::now(), 
            dt : f32::INFINITY,
        }
    }

    fn draw_frame(&mut self, display: &glium::Display<glium::glutin::surface::WindowSurface>) {


        self.mesh.load_into_vertex_buffer(& self.draw_context.vertex_buffer);
        let vertex_buffer = &self.draw_context.vertex_buffer;
        let indices = self.draw_context.indices;
        let programs = &self.draw_context.program;

        let mut target = display.draw();
        target.clear_color(0.1, 0.1, 0.1, 1.0);

        
        let uniforms = uniform! {
            screen_size: target.get_dimensions()
        };
        target.draw(vertex_buffer, indices, &programs, &uniforms,
            &Default::default()).unwrap();
        target.finish().unwrap();

        
        
    }

    fn update(&mut self) {
        let now = std::time::Instant::now();
        self.dt = now.duration_since(self.time).as_secs_f32();
        self.time=now;

        self.mesh.rotate_z(1.*self.dt);

        
    }

    
}