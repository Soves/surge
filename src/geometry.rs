
//define our vertex struct
#[derive(Copy, Clone)]

pub struct Vertex
{
    pub position: [f32; 2],
    pub tex_coords: [f32; 2]
}

//implement the certex with glium
implement_vertex!(Vertex, position, tex_coords);


#[derive(Copy, Clone)]
pub struct Triangle
{
    pub vertices: [Vertex; 3],
}

impl Triangle
{
    pub fn from(points: [f32; 6]) -> Triangle
    {
        
        Triangle
        {
            vertices:
            [
                Vertex
                {
                    position: [points[0], points[1]],
                    tex_coords: [0.0, 0.0]
                },
                Vertex
                {
                    position: [points[2], points[3]],
                    tex_coords: [0.0, 0.0]
                },
                Vertex
                {
                    position: [points[4], points[5]],
                    tex_coords: [0.0, 0.0]
                },
            ]
        }
    }

    pub fn to_trianglelist(&self) -> Vec<Vertex>
    {
        
        vec![
            self.vertices[0],
            self.vertices[1],
            self.vertices[2]
        ]
        
    }
}

#[derive(Copy, Clone)]
pub struct Rectangle
{
    pub triangles: [Triangle; 2],
}

impl Rectangle
{
    pub fn from(points: [f32; 4]) -> Rectangle
    {
        Rectangle
        {
            triangles:
            [
                Triangle
                {
                    vertices:
                    [
                        Vertex
                        {
                            position: [points[0], points[1]],
                            tex_coords: [0.0, 0.0]
                        },
                        Vertex
                        {
                            position: [points[2], points[1]],
                            tex_coords: [1.0, 0.0]
                        },
                        Vertex
                        {
                            position: [points[0], points[3]],
                            tex_coords: [0.0, 1.0]
                        },
                    ]
                },
                Triangle
                {
                    vertices:
                    [
                        Vertex
                        {
                            position: [points[2], points[1]],
                            tex_coords: [1.0, 0.0]
                        },
                        Vertex
                        {
                            position: [points[2], points[3]],
                            tex_coords: [1.0, 1.0]
                        },
                        Vertex
                        {
                            position: [points[0], points[3]],
                            tex_coords: [0.0, 1.0]
                        },
                    ]
                }
            ]
        }
        
    }

    pub fn to_trianglelist(&self) -> Vec<Vertex>
    {
        
        let mut list = self.triangles[0].to_trianglelist();
        let mut list_2 = self.triangles[1].to_trianglelist();
        list.append(&mut list_2);
        
        list
    }
}

//TODO: handle other geometry stuff here