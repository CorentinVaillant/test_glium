use glium::{glutin::surface::WindowSurface, winit::{self, application::ApplicationHandler}, Display};


pub trait ApplicationContext{
    fn draw_frame(&mut self, _display: &Display<WindowSurface>){()}
    fn new(display: &Display<WindowSurface>)->Self;
    fn update(&mut self){ }
    fn handle_window_event(&mut self, _event: &glium::winit::event::WindowEvent, _window: &glium::winit::window::Window) { }
}

pub struct State<T>{
    pub display: glium::Display<WindowSurface>,
    pub window: glium::winit::window::Window,
    pub context: T
}

pub struct App<T>{
    state: Option<State<T>>,
    app_name: &'static str
}

impl<T> App<T> {
    pub fn new(app_name:&'static str)->Self{
        App{state:None,app_name}
    }
}

impl<T:ApplicationContext> State<T> {
    pub fn new( event_loop: &glium::winit::event_loop::ActiveEventLoop, window_title :&str)->Self{
        let (window,display) = glium::backend::glutin::SimpleWindowBuilder::new()
            .with_title(window_title)
            .build(event_loop);

        Self::from_display_window(display, window)
    }

    pub fn from_display_window(
        display:glium::Display<WindowSurface>,
        window :glium::winit::window::Window
    )->Self{
        let context = T::new(&display);
        Self { display, window, context }
    }
#[allow(dead_code)]
    pub fn run(){
        let event_loop = glium::winit::event_loop::EventLoop::builder()
            .build()
            .expect("event loop building in State::run_loop()");
        let mut app =App::<T>{
            state: None,
            app_name: "My App" //TODO
        };

        let result = event_loop.run_app(&mut app);
        result.unwrap();
    }

}

impl<T: ApplicationContext> ApplicationHandler<()> for App<T>{
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        println!("[app handeler] : resumed");
        self.state = Some(State::new(event_loop, &self.app_name));
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            //---Resize---
            glium::winit::event::WindowEvent::Resized(new_size)=>{
                if let Some(state) =&mut self.state{
                    state.display.resize(new_size.into());
                }
            },

            //---RedrawRequested---
            glium::winit::event::WindowEvent::RedrawRequested=>{
                if let Some(state)= &mut self.state {
                    state.context.update();
                    state.context.draw_frame(&state.display);
                }
            },

            //---CloseRequested---
            glium::winit::event::WindowEvent::CloseRequested
            | glium::winit::event::WindowEvent::KeyboardInput { event: glium::winit::event::KeyEvent {
                state: glium::winit::event::ElementState::Pressed,
                logical_key: glium::winit::keyboard::Key::Named(glium::winit::keyboard::NamedKey::Escape),
                ..
            }, ..} => {
                event_loop.exit()
            },

            ev=>{
                if let Some(state) = &mut self.state {
                    state.context.handle_window_event(&ev, &state.window);
                }
            }
        }
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Some(state) = &self.state {
            state.window.request_redraw();
        }
    }
}