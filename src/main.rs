#[macro_use]
extern crate glium;

pub mod geometry;

fn main() 
{
    use glium::{glutin, Surface};
    use geometry::{Vertex};

    //define neccessary variables for rendering
    //glutins event loop so we can hanle window events and render things
    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new(); //uses glutin to build a window
    let cb = glutin::ContextBuilder::new(); //uses gluting to build a context

     //finally make a display for glium which ties together window with the event loop
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();


    //TEMP RENDERING STUFF

    //define triangle
    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [ 0.0,  0.5] };
    let vertex3 = Vertex { position: [ 0.5, -0.25] };
    let shape = vec![vertex1, vertex2, vertex3];

    //create vertex buffer from trianle
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    //shaders
    let vertex_shader_src = r#"

        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;
    
    let fragment_shader_src = r#"

        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    //combine shaders into a program
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    //pass anonymous function/closure as a callback to glutins event loop
    event_loop.run(move |ev, _, control_flow| {

        //gets our double buffered surface for drawing onto
        let mut target = display.draw();

        //clear the target with a solid color
        target.clear_color(0.0,0.0,1.0,1.0);

        //draw stuff
        target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
            &Default::default()).unwrap();

        //swap our double buffered surface to the display so it will be visible on the window
        //after we can no longer draw onto it
        target.finish().unwrap();

        //calculate time for next frame and set the control flow variable to the time
        //so the event loop executes again at that specified time
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        //match event loops event type
        //so we can exit the loop when requested
        match ev {
            //if event is a window event
            glutin::event::Event::WindowEvent { event, .. } => match event {
                //handle close request and exit the loop
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },

            //ignore rest of the events
            _ => (),
        }
    });

}