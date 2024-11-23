use glium::{glutin::surface::WindowSurface, index::NoIndices, winit::{application::ApplicationHandler, event_loop::{ActiveEventLoop, EventLoop}, window::Window}, Display, Program, VertexBuffer};

use crate::meshes::Vertex;


pub type InitDraw<UsrEnv>       =fn(&glium::winit::event_loop::EventLoop<()>  ,&AppEnv,&mut UsrEnv)->DrawEnv;
pub type UpdateDraw<UsrEnv>     =fn(&glium::winit::event_loop::ActiveEventLoop,&AppEnv,&mut UsrEnv,&mut DrawEnv)->();

pub type UsrInit<UsrEnv>        =fn(&glium::winit::event_loop::ActiveEventLoop,&mut UsrEnv,&AppEnv)->();
pub type UsrUpdate<UsrEnv>      =fn(&glium::winit::event_loop::ActiveEventLoop,&mut UsrEnv,&AppEnv)->();

pub struct MyApp<UsrEnv>{
    
    //graphics
    _init_draw   :InitDraw<UsrEnv>,
    update_draw :UpdateDraw<UsrEnv>,

    //usr funcs
    usr_init    :UsrInit<UsrEnv>,
    usr_update  :UsrUpdate<UsrEnv>,

    //envs
    app_env : AppEnv,
    draw_env: DrawEnv,
    usr_env : UsrEnv,
}

pub struct AppEnv{
    pub time: std::time::Instant,
    pub dt  :f32
}

impl AppEnv{
    ///update the current time, and delta time
    fn update_time(&mut self){
        let now = std::time::Instant::now();
        self.dt = now.duration_since(self.time).as_secs_f32();
        self.time=now;
    }
}

pub struct DrawEnv{
    pub vertex_buffer : VertexBuffer<Vertex>,
    pub programs : Program,
    pub indices :NoIndices,

    pub display :Display<WindowSurface>,
    pub _window  :Window,
}


impl<UsrEnv> MyApp<UsrEnv> {
    pub fn new(event_loop:&EventLoop<()>,init_draw :InitDraw<UsrEnv>,update_draw :UpdateDraw<UsrEnv>,usr_init :UsrInit<UsrEnv>,usr_update :UsrUpdate<UsrEnv>,usr_env:UsrEnv)->Self{
        let app_env = AppEnv{
            time: std::time::Instant::now(),
            dt  : f32::MAX,
        };
        let mut usr_env= usr_env;
        let draw_env = init_draw(event_loop,&app_env,&mut usr_env);
        
        Self{
            _init_draw: init_draw  ,
            update_draw,
            usr_init   ,
            usr_update ,

            app_env,
        
            draw_env ,
            usr_env,

        }
    }

    pub fn draw(&mut self,event_loop:&ActiveEventLoop){
        (self.update_draw)(event_loop,&self.app_env,&mut self.usr_env,&mut self.draw_env);
    }
}



impl<UsrEnv> ApplicationHandler for MyApp<UsrEnv>{
    fn resumed(&mut self, _event_loop: &glium::winit::event_loop::ActiveEventLoop) {
        println!("resumed");
    }
    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        ()
    }
    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        println!("exiting")
    }
    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        println!("suspended")    
    } 
    

    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: glium::winit::event::StartCause) {
        match cause {
            glium::winit::event::StartCause::Init => {
                (self.usr_init)(event_loop,&mut self.usr_env, &self.app_env);
            },
            glium::winit::event::StartCause::Poll => {
                self.app_env.update_time();
                (self.usr_update)(event_loop,&mut self.usr_env,&self.app_env);
                self.draw(event_loop);

            },
            glium::winit::event::StartCause::ResumeTimeReached { start:_, requested_resume:_ } => (),
            glium::winit::event::StartCause::WaitCancelled { start:_, requested_resume:_ } => (),
        }
    }

    fn window_event(
        &mut self,
        event_loop: &glium::winit::event_loop::ActiveEventLoop,
        _window_id: glium::winit::window::WindowId,
        event: glium::winit::event::WindowEvent,
    ) {
        match event {
            glium::winit::event::WindowEvent::CloseRequested => event_loop.exit(),
            glium::winit::event::WindowEvent::Destroyed => println!("the windows as been destroy"),
            glium::winit::event::WindowEvent::RedrawRequested => self.draw(event_loop),

            _=>()
        }
    }
}