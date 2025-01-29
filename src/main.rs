

use my_glium_util::app::{App, ApplicationContext};
use glium::{uniform, winit::event_loop::EventLoop, Surface};
use my_glium_util::camera::OrthographicCam;
use my_glium_util::mesh::mesh::Mesh;
use my_glium_util::mesh::obj_parser::WaveFrontParsable;
use my_glium_util::object_traits::{ApplicableSceneObject, Renderable, SceneObject, Translation};


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
    program: glium::Program,
}


struct GlobalVariable{
    cam_pos : Translation,
}
impl Default for GlobalVariable{
    fn default() -> Self {
        Self { cam_pos: Translation::zero() }
    }
}
struct MyObjects{
    triangle : Mesh,
    camera : OrthographicCam,
}

impl Default for MyObjects {
    fn default() -> Self {
        let mut triangle = Mesh::load_from_wavefront(OBJ_PATH).unwrap();
        triangle.scale([100.;3].into());
        triangle.apply_scale();
        Self{
            triangle ,
            camera : OrthographicCam::new([0.,-5.,0.].into(), 1. , -1., -10. , -10. ,)
        }
    }
}

struct MyApp {
    objects:MyObjects,
    variable : GlobalVariable,

    draw_context : DrawContext,

    pub time: std::time::Instant,
    pub dt  :f32
}

impl ApplicationContext for MyApp {
    fn new(display: &glium::Display<glium::glutin::surface::WindowSurface>)->Self {


        let vertex_shader_src = std::fs::read_to_string("shaders/vertex.vert").unwrap();
        let fragment_shader_src = std::fs::read_to_string("shaders/fragment.frag").unwrap();


        let program= glium::Program::from_source(display, &vertex_shader_src, &fragment_shader_src, None).unwrap();

        MyApp { 
            objects : MyObjects::default(),
            variable : GlobalVariable::default(),
            draw_context: DrawContext{
                program,
            },

            time : std::time::Instant::now(), 
            dt : f32::INFINITY,
        }
    }

    fn draw_frame(&mut self, display: &glium::Display<glium::glutin::surface::WindowSurface>) {

        let mut target = display.draw();
        target.clear_color(0.1, 0.1, 0.1, 1.0);

        let programs = &self.draw_context.program;
        
        

        let uniforms = uniform! {
            screen_size: target.get_dimensions(),
            projection_matrix : self.objects.camera,
        };

        let draw_parameters = glium::DrawParameters::default();

        self.objects.triangle.render(display, &programs, &mut target, &uniforms, &draw_parameters).unwrap_or_else(|e|{dbg!(e);});

        target.finish().unwrap_or_else(|e|{dbg!(e);});
        
    }

    fn update(&mut self) {
        let now = std::time::Instant::now();
        self.dt = now.duration_since(self.time).as_secs_f32();
        self.time=now;


        self.variable.cam_pos += Translation::from([0.,0.,0.]);
        self.objects.camera.translate(self.variable.cam_pos);
        
    }

    
}