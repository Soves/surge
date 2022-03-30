use glium::{Surface, VertexBuffer, uniforms, texture};
use crate::geometry::{Vertex, Rectangle};

pub struct TexturePage
{
    texture: texture::SrgbTexture2d,

}

pub struct Texture{

    uv: Rectangle

    

}

impl Texture
{


    pub fn new(uv: Rectangle)
    {
        
        Texture
        {
            uv,
        }

    }

    
}