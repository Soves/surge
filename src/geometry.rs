
//define our vertex struct
#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
}

//implement the certex with glium
implement_vertex!(Vertex, position);

//TODO: handle other geometry stuff here