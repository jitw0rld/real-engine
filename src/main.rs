#[macro_use]
extern crate glium;

fn main() {
    use glium::{glutin, Surface};

    // Create event loop
    let event_loop = glium::glutin::event_loop::EventLoop::new();

    // Create window context
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_title("Hello world!")
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(1024.0, 768.0));

    // Create OpenGL contexts
    let cb = glium::glutin::ContextBuilder::new();

    // Create display
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    // Define vertex structure
    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    // Implement vertex structure
    implement_vertex!(Vertex, position);

    // Initialize vertex shader
    let vertex_shader_src = include_str!("shaders/vertex.glsl");

    // Initialize fragment shader
    let fragment_shader_src = include_str!("shaders/fragment.glsl");

    // Define rendering program
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    // Create frame buffer/event loop
    event_loop.run(move |ev, _, control_flow| {

        // Frame buffer
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        
        // Event loop
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            _ => (),
        }

        // Draw frame
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.finish().unwrap();

    });
}