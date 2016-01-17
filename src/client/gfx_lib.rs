use gfx::shade::TextureParam;

gfx_vertex!( Vertex {
    a_pos@ a_pos: [f32; 3],
    a_color@ a_color: [f32; 4],
});

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32, color: [f32; 4]) -> Vertex {
        Vertex {
            a_pos: [x, y, z],
            a_color: color,
        }
    }
}

gfx_parameters!( Params {
    u_model_view_proj@ u_model_view_proj: [[f32; 4]; 4],
    t_color@ t_color: TextureParam<R>,
});
