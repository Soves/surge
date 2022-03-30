

pub struct Point2D
{
    pub x: f32,
    pub y: f32
}


pub struct Transform2D
{
    pub position: Point2D,
    pub scale: f32,
    pub rotation: f32
}

impl Transform2D
{

    pub fn new(x: f32, y: f32, scale: f32, rotation: f32) -> Transform2D
    {
        Transform2D
        {
            position: Point2D
            {
                x,
                y
            },
            scale,
            rotation
        }
    }

}