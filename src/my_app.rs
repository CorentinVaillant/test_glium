use glium::winit::application::ApplicationHandler;
pub struct MyApp<DrawF,UpdaF>
where 
    DrawF:Fn(&glium::winit::event_loop::ActiveEventLoop),
    UpdaF:Fn(&glium::winit::event_loop::ActiveEventLoop), 
{
    draw_func :DrawF,
    update_func:UpdaF,

}

impl<DrawF,UpdaF> MyApp<DrawF,UpdaF> 
where 
    DrawF:Fn(&glium::winit::event_loop::ActiveEventLoop),
    UpdaF:Fn(&glium::winit::event_loop::ActiveEventLoop),
{
    pub fn new(draw_func:DrawF,update_func:UpdaF)->Self{
        return MyApp{draw_func,update_func};
    }
}

impl<DrawF,UpdaF> ApplicationHandler for MyApp<DrawF,UpdaF>
where 
    DrawF:Fn(&glium::winit::event_loop::ActiveEventLoop),
    UpdaF:Fn(&glium::winit::event_loop::ActiveEventLoop),
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

    fn new_events(&mut self, event_loop: &glium::winit::event_loop::ActiveEventLoop, _cause: glium::winit::event::StartCause) {
        (self.update_func)(event_loop);
        
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
            glium::winit::event::WindowEvent::RedrawRequested=>(self.draw_func)(event_loop),
            glium::winit::event::WindowEvent::MouseInput { device_id, state, button }=>{
                
            }
            
            _=>println!("event :{:?}",event),
        }

    }
}