
use glium::{Surface, VertexBuffer, uniforms, texture};
use crate::geometry::{Vertex, Rectangle};




pub struct Sprite
{

    pub texture: glium::texture::SrgbTexture2d,
    pub rectangle: Rectangle,
    pub vertex_buffer: VertexBuffer<Vertex>,
    pub indices: glium::index::NoIndices,
    pub program: glium::Program
}

impl Sprite
{

    pub fn new( display: &glium::Display, path: &str) -> Sprite
    {
        let image = image::open(path).unwrap().to_rgba8();
        let image_dimensions = image.dimensions();
        let image = texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

        let rect = Rectangle::from([-0.5, -0.5,  0.5, 0.5]);

        //shaders
        let vertex_shader_src = r#"

            #version 140

            in vec2 position;
            in vec2 tex_coords;

            out vec2 v_position;
            out vec2 v_tex_coords;

            void main() {
                v_position = position;
                v_tex_coords = tex_coords;

                gl_Position = vec4(position, 0.0, 1.0);
            }
        "#;

        let fragment_shader_src = r#"

            #version 140

            out vec4 color;
            in vec2 v_position;
            in vec2 v_tex_coords;

            uniform sampler2D tex;

            void main() {
                color = texture(tex, v_tex_coords);
            }
        "#;

        Sprite
        {
            texture: texture::SrgbTexture2d::new(display, image).unwrap(),
            rectangle: rect,
            vertex_buffer: VertexBuffer::new(display, &rect.to_trianglelist()).unwrap(),
            indices: glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
            program: glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap()
        }
    }

    pub fn draw(&self, target: &mut glium::Frame)
    {
        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [ 0.0 , 0.0, 0.0, 1.0f32],
            ],
            tex: &self.texture,
        };

        target.draw(&self.vertex_buffer, &self.indices, &self.program, &uniforms,
            &Default::default()).unwrap();
    }

}