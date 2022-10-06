#[macro_use]
extern crate glium;
mod world;

fn main() {
    use glium::{glutin, Surface};

    // Create event loop
    let event_loop = glium::glutin::event_loop::EventLoop::new();

    // Create window context
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_title("Hello world!")
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(1024.0, 768.0));

    // Create OpenGL contexts
    let cb = glium::glutin::ContextBuilder::new().with_depth_buffer(24);

    // Create display
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    // Initialize vertex shader
    let vertex_shader_src = include_str!("shaders/vertex.glsl");

    // Initialize fragment shader
    let fragment_shader_src = include_str!("shaders/fragment.glsl");

    // Load world data
    let positions = glium::VertexBuffer::new(&display, &world::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &world::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &world::INDICES).unwrap();

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
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        // Create frame in buffer
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
        
        // Initialize geometry matrix
        let matrix = [
            [0.5, 0.0, 0.0, 0.0],
            [0.0, 0.5, 0.0, 0.0],
            [0.0, 0.0, 0.5, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ];
        
        // Initialize perspective matrix
        let perspective = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = width as f32 / height as f32;

            let fov: f32 = 3.141592 / 3.0;
            let zfar = 1024.0;
            let znear = 0.1;

            let f = 1.0 / (fov / 2.0).tan();

            [
                [f * aspect_ratio, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, (zfar + znear) / (znear - zfar), -1.0],
                [0.0, 0.0, 2.0 * zfar * znear / (znear - zfar), 0.0f32],
            ]
        };

        // Initialize lighting
        let light = [-1.0, 0.4, 0.9f32];

        // Define frame rendering parameters
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        // Draw frame
        target.draw((&positions, &normals), &indices, &program, &uniform! { matrix: matrix, perspective: perspective, light: light }, &params).unwrap();
        target.finish().unwrap();

    });
}