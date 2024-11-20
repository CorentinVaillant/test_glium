use glium::glutin::surface::WindowSurface;
use glium::index::NoIndices;
use glium::{winit::application::ApplicationHandler, VertexBuffer};
use glium::{Display, Program};

use crate::meshes::Vertex;
pub struct MyApp<DrawF,InitF,UpdaF>
where 
    DrawF:Fn(&glium::winit::event_loop::ActiveEventLoop,&mut AppAttr),
    InitF:Fn(&glium::winit::event_loop::ActiveEventLoop,&mut AppAttr),  
    UpdaF:Fn(&glium::winit::event_loop::ActiveEventLoop,f32,&mut AppAttr),
{
    draw_func   :DrawF,
    update_func :UpdaF,
    init_func   :InitF,

    app_attributs:AppAttr
}

pub struct AppAttr{
    //current instant (use to make delta)
    pub time:std::time::Instant,

    pub vertex_buffer :VertexBuffer<Vertex>,
    pub indices :NoIndices,

    pub shader_program:Option<Program>,
}


impl<DrawF,InitF,UpdaF> MyApp<DrawF,InitF,UpdaF> 
where 
    DrawF:Fn(&glium::winit::event_loop::ActiveEventLoop,&mut AppAttr),
    InitF:Fn(&glium::winit::event_loop::ActiveEventLoop,&mut AppAttr),  
    UpdaF:Fn(&glium::winit::event_loop::ActiveEventLoop,f32,&mut AppAttr),
{
    pub fn new(
        draw_func: DrawF,init_func:InitF,update_func:UpdaF ,
        display:&Display<WindowSurface>)->Self{
        return MyApp{
            draw_func,init_func,update_func,

            app_attributs:AppAttr{
                time:std::time::Instant::now(),
                vertex_buffer:VertexBuffer::new(display, &[]).unwrap(),
                indices:glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan),
                shader_program:None,
            },
        };
    }
}

impl<DrawF,InitF,UpdaF> ApplicationHandler for MyApp<DrawF,InitF,UpdaF>
where 
    DrawF:Fn(&glium::winit::event_loop::ActiveEventLoop,&mut AppAttr),
    InitF:Fn(&glium::winit::event_loop::ActiveEventLoop,&mut AppAttr),  
    UpdaF:Fn(&glium::winit::event_loop::ActiveEventLoop,f32,&mut AppAttr),
{

    fn resumed(&mut self, _event_loop: &glium::winit::event_loop::ActiveEventLoop) {
        println!("resumed")
    }

    fn suspended(&mut self, _event_loop: &glium::winit::event_loop::ActiveEventLoop) {
        println!("suspended");
    }

    
    fn exiting(&mut self, _event_loop: &glium::winit::event_loop::ActiveEventLoop) {
        println!("exiting...")
    }

    fn new_events(&mut self, event_loop: &glium::winit::event_loop::ActiveEventLoop, cause: glium::winit::event::StartCause) {
        match cause {
            glium::winit::event::StartCause::Init => {
                todo!() //TODO init
            },
            glium::winit::event::StartCause::Poll => {

                let now =std::time::Instant::now(); 
                let delta = now.duration_since(self.app_attributs.time).as_secs_f32();
                self.app_attributs.time = now;
        
                (self.update_func)(event_loop,delta,&mut self.app_attributs);
            },
            
            _=>(),
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
            glium::winit::event::WindowEvent::AxisMotion { device_id: _, axis:_, value:_ }=>(),
            glium::winit::event::WindowEvent::CursorMoved { device_id:_, position:_ }=>(),
            glium::winit::event::WindowEvent::CursorEntered { device_id :_}=>(),
            glium::winit::event::WindowEvent::CursorLeft { device_id:_ }=>(),
            glium::winit::event::WindowEvent::RedrawRequested=>(self.draw_func)(event_loop,&mut self.app_attributs),
            glium::winit::event::WindowEvent::MouseInput { device_id:_, state:_, button:_ }=>(),
            
            _=>println!("event :{:?}",event),
        }

    }
}