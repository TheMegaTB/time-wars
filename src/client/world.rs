use time::PreciseTime;
use gfx::render::mesh::Slice;
use gfx::render::mesh::Mesh;
use gfx::render::mesh::ToIndexSlice;
use gfx::extra::factory::FactoryExt;
use gfx_device_gl::{Factory, Resources};
use gfx::PrimitiveType::TriangleList;

// ------------------- File -------------------
use std::fs::File;
use std::io::Read;

use rand::distributions::{IndependentSample, Range};
use rand;

use math_fx::{calculate_bezier, max, min};
use gfx_lib::Vertex;
use consts::*;

use vecmath::{
    Vector3,
    vec3_cross,
    vec3_dot,
    Vector2
};


pub struct TimeDiff {
    start: PreciseTime
}

impl TimeDiff {
    pub fn start() -> TimeDiff {
        TimeDiff {
            start: PreciseTime::now()
        }
    }

    pub fn end(&self) {
        println!("{:?}", (self.start.to(PreciseTime::now()).num_nanoseconds().unwrap() as TTime)/1000000000.0);
    }
}


pub struct StaticWorldObj {
    pub model: T4Matrix<TCoordinate>,
    pub animation_id: usize,
    pub animation_time: TTime,
    pub position: Vector2<TCoordinate>
}

impl StaticWorldObj {
    fn new(pos: Vector2<TCoordinate>, rot: TRotation, animation_id: usize, animation_time: TTime) -> StaticWorldObj  {
        StaticWorldObj {
            model: [
                [rot.cos(), 0.0, -rot.sin(), 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [rot.sin(), 0.0, rot.cos(), 0.0],
                [pos[0], 0.0, pos[1], 1.0]
            ],
            animation_id: animation_id,
            animation_time: animation_time,
            position: pos
        }
    }
}

struct DynamicWorldObj {
    position: Vector2<TCoordinate>,
    rotation: TRotation,
    animation_id: usize,
    animation_time: TTime,
    animation_start_time: TTime
}

impl DynamicWorldObj {
    fn new(pos: Vector2<TCoordinate>, rot: TRotation, animation_id: usize, animation_time: TTime, animation_start_time: TTime) -> DynamicWorldObj {
        DynamicWorldObj {
            position: pos,
            rotation: rot,
            animation_id: animation_id,
            animation_time: animation_time,
            animation_start_time: animation_start_time
        }
    }

    fn get_model(&mut self) -> T4Matrix<TCoordinate> {
        [
            [self.rotation.cos(), 0.0, -self.rotation.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [self.rotation.sin(), 0.0, self.rotation.cos(), 0.0],
            [self.position[0], 0.0, self.position[1], 1.0]
        ]
    }
}

struct FloorObj {
    y: TCoordinate
}

pub struct AnimationObj {
    points: Vec<Vec<Vector3<TCoordinate>>>,
    materials: Vec<[TColor; 4]>,
    pub slice: Slice<Resources>,
    collision_radius: TCoordinate,
    collision_y: TCoordinate
}

impl AnimationObj {
    fn from_file(filename: String, factory: &mut Factory) -> AnimationObj {
        let mut file = File::open(filename).unwrap();
        let mut contents: Vec<u8> = Vec::new();
        file.read_to_end(&mut contents).unwrap();
        let filestr = String::from_utf8(contents).unwrap();

        let lines = filestr.split("\n").collect::<Vec<&str>>();
        let mut slice_vec: Vec<u32> = Vec::new();

        let mut key_points = Vec::new();
        let mut materials = Vec::new();

        let mut current_material = [0.0, 0.0, 0.0, 1.0];
        let material_range = Range::new(0.3, 1.0);
        let mut rng = rand::thread_rng();

        for line in lines.iter()
        {
            let parts = line.split(" ").collect::<Vec<&str>>();
            if parts[0] == "g" {
                for i in 0..3 {
                    current_material[i] = parts[i+1].parse::<TColor>().unwrap();
                }
            }
            else if parts[0] == "v" {
                let x = parts.iter().skip(1).enumerate().filter_map(|(n, x): (usize, &&str)| if n % 3 == 0 {Some(x.parse::<TCoordinate>().unwrap())} else {None});
                let y = parts.iter().skip(1).enumerate().filter_map(|(n, y): (usize, &&str)| if n % 3 == 1 {Some(y.parse::<TCoordinate>().unwrap())} else {None});
                let z = parts.iter().skip(1).enumerate().filter_map(|(n, z): (usize, &&str)| if n % 3 == 2 {Some(z.parse::<TCoordinate>().unwrap())} else {None});

                let point: Vec<Vector3<TCoordinate>> = x.zip(y).zip(z).map(|((x,y), z)| [x,y,z]).collect();

                key_points.push(point);
                let colormulti = material_range.ind_sample(&mut rng);
                materials.push([current_material[0] * colormulti, current_material[1] * colormulti, current_material[2] * colormulti, current_material[3]]);
            }
            else if parts[0] == "f" {
                let p1 = parts[1].parse::<usize>().unwrap();
                let p2 = parts[2].parse::<usize>().unwrap();
                let p3 = parts[3].parse::<usize>().unwrap();
                slice_vec.push(p1 as u32);
                slice_vec.push(p2 as u32);
                slice_vec.push(p3 as u32);
            }
        }

        let mut points = Vec::new();

        let slice = slice_vec[..].to_slice(factory, TriangleList);
        AnimationObj {
            points: points,
            materials: materials,
            slice: slice,
            collision_radius: lines[0].parse::<TCoordinate>().unwrap(),
            collision_y: lines[1].parse::<TCoordinate>().unwrap()
        }
    }

    pub fn get_meshs(&self, t: TTime, animation_duration: TTime, factory: &mut Factory) -> Mesh<Resources> {
        let dt = (t % animation_duration) / animation_duration;

        let mut vertex_data = Vec::new();

        for (i, p) in self.points.iter().enumerate() {
            let mut q = p.clone();
            let nx = 2.0 * p[0][0] - p[1][0];
            let ny = 2.0 * p[0][1] - p[1][1];
            let nz = 2.0 * p[0][2] - p[1][2];
            q.push([nx, ny, nz]);
            q.push(p[0]);

            let x = calculate_bezier(dt, q.iter().map(|x| x[0]).collect());
            let y = calculate_bezier(dt, q.iter().map(|x| x[1]).collect());
            let z = calculate_bezier(dt, q.iter().map(|x| x[2]).collect());
            vertex_data.push(Vertex::new(x, y, z, self.materials[i]));
            // vertex_data.push(Vertex::new(p[0][0], p[0][1], p[0][2], self.materials[i]));
        }

        factory.create_mesh(&vertex_data)
    }
}

pub struct Player {
    pub moving: [TCoordinate; 6],
    yaw: TRotation,
    pitch: TRotation,
    y_s: TRotation,
    y_c: TRotation,
    position: Vector3<TCoordinate>,
    up: Vector3<TCoordinate>,
    right: Vector3<TCoordinate>,
    forward: Vector3<TCoordinate>,
    direction: Vector3<TCoordinate>,
    y_speed: TCoordinate
}

impl Player {
    fn new(x: TCoordinate, y: TCoordinate, z: TCoordinate) -> Player {
        Player {
            moving: [0.0; 6],
            yaw: 0.0,
            pitch: 0.0,
            y_s: 0.0,
            y_c: 1.0,
            position: [x, y, z],
            right:   [1.0, 0.0, 0.0],
            up:      [0.0, 1.0, 0.0],
            forward: [0.0, 0.0, 1.0],
            direction: [0.0, 0.0, 0.0],
            y_speed: 0.0
        }
    }

    fn update_right(&mut self) {
        self.right = vec3_cross(self.up, self.forward);
    }

    pub fn update_directions(&mut self) {
        self.direction = [
            (self.moving[0]-self.moving[1]) * self.y_s + (self.moving[2]-self.moving[3]) * self.y_c,
            (self.moving[0]-self.moving[1]) * self.y_c - (self.moving[2]-self.moving[3]) * self.y_s,
            self.moving[4] - self.moving[5]
        ];
    }

    pub fn rotate_player(&mut self, yaw: TRotation, pitch: TRotation) {
        let h_pi = 3.1415926/2.0;
        self.yaw -= yaw;
        self.pitch = min(max(self.pitch + pitch, -h_pi), h_pi);
        let (y_s, y_c, p_s, p_c) = (self.yaw.sin(), self.yaw.cos(), self.pitch.sin(), self.pitch.cos());
        self.y_s = y_s;
        self.y_c = y_c;
        self.forward = [y_s * p_c, p_s, y_c * p_c];
        self.up = [y_s * -p_s, p_c, y_c * -p_s];
        self.update_right();
        self.update_directions();
    }

    pub fn move_player(&mut self, id: usize, value: TCoordinate) {
        self.moving[id] = value;
        self.update_directions();
    }
}

pub fn check_collision(player: &mut Player, collision_radius: f32, collision_y: f32, position: &Vector2<TCoordinate>) {
    if player.position[1] < collision_y+PLAYER_HEIGHT {
        let dx = player.position[0]-position[0];
        let dz = player.position[2]-position[1];
        let len = (dx * dx + dz * dz).sqrt();
        if len < collision_radius*0.9 {
            player.position[0] = position[0] + dx*1.1;
            player.position[2] = position[1] + dz*1.1;
            player.y_speed = 0.0;
        }
        else if len < collision_radius {
            player.position[0] = position[0] + dx/len*collision_radius;
            player.position[2] = position[1] + dz/len*collision_radius;
        }
    }
}

pub struct World {
    pub animations: Vec<AnimationObj>,
    pub in_game_time: TTime,
    pub player: Player,
    pub static_world_objects: Vec<StaticWorldObj>,
    dynamic_world_objects: Vec<DynamicWorldObj>,
    floor_objects: Vec<FloorObj>,
    last_time: PreciseTime  //TODO remove and impl server
}

impl World {
    #[allow(dead_code)]
    pub fn new(x: TCoordinate, y: TCoordinate, z: TCoordinate) -> World {
        World {
            animations: Vec::new(),
            in_game_time: 0.0,
            player: Player::new(x, y, z),
            static_world_objects: Vec::new(),
            dynamic_world_objects: Vec::new(),
            floor_objects: Vec::new(),
            last_time: PreciseTime::now()
        }
    }

    pub fn example() -> World {
        World {
            animations: Vec::new(),
            in_game_time: 0.0,
            player: Player::new(0.0, PLAYER_HEIGHT, 4.0),
            static_world_objects: vec![
                StaticWorldObj::new([-1.0, 0.0], 0.0, 0, 3.0),
                StaticWorldObj::new([-2.0, 0.0], 1.0, 1, 4.0),
                StaticWorldObj::new([1.0, 0.0], 2.0, 0, 3.0),
                StaticWorldObj::new([2.0, 0.0], 3.0, 0, 3.0),
                StaticWorldObj::new([3.0, 0.0], 4.0, 0, 3.0),
                StaticWorldObj::new([4.0, 0.0], 5.0, 0, 3.0),
                StaticWorldObj::new([5.0, 0.0], 6.0, 0, 3.0),
                StaticWorldObj::new([6.0, 0.0], 7.0, 0, 3.0)
            ],
            dynamic_world_objects: Vec::new(),
            floor_objects: Vec::new(),
            last_time: PreciseTime::now()
        }
    }

    pub fn load_animations(&mut self, assets_path: String, n: u32, factory: &mut Factory) {
        for i in 0..n {
            self.animations.push(AnimationObj::from_file(assets_path.clone() + "/3d/" + &(i.to_string())[..] + ".3d", factory));
        }
    }

    pub fn update(&mut self, factory: &mut Factory) -> Vec<(Mesh<Resources>, Slice<Resources>, T4Matrix<TCoordinate>)> { //TODO: remove dt and impl server comunication
        let now = PreciseTime::now();
        let dt = (self.last_time.to(now).num_nanoseconds().unwrap() as TTime)/1000000000.0;
        self.last_time = now;
        self.in_game_time += dt;

        // --------- update position ---------
        //let old_pos = self.player.position.clone();

        if self.player.moving[4] == 0.0 {
            self.player.y_speed = max(self.player.y_speed - 6.0 * dt, -8.0);
        }
        else {
            self.player.y_speed = min(self.player.y_speed + self.player.direction[2]  * dt, 4.0);
        }

        if self.player.position[1] <= PLAYER_HEIGHT && self.player.y_speed < 0.0{
            self.player.position[1] = PLAYER_HEIGHT;
            self.player.y_speed = 0.0;
            self.player.position[0] -= self.player.direction[0] * dt;
            self.player.position[2] -= self.player.direction[1] * dt;
        }
        else {
            self.player.position[0] -= self.player.direction[0] * dt * 1.5;
            self.player.position[2] -= self.player.direction[1] * dt * 1.5;
        }
        self.player.position[1] += self.player.y_speed * dt;


        let mut result: Vec<(Mesh<Resources>, Slice<Resources>, T4Matrix<TCoordinate>)> = Vec::new();
        for p in self.static_world_objects.iter() {
            check_collision(&mut self.player, self.animations[p.animation_id].collision_radius, self.animations[p.animation_id].collision_y, &(p.position));
            result.push((self.animations[p.animation_id].get_meshs(self.in_game_time, p.animation_time, factory), self.animations[p.animation_id].slice.clone(), p.model));
        }

        result
    }

    pub fn get_view_matrix(&mut self) -> T4Matrix<TCoordinate>{
        let p = self.player.position;
        let r = self.player.right;
        let u = self.player.up;
        let f = self.player.forward;
        [
            [r[0], u[0], f[0], 0.0],
            [r[1], u[1], f[1], 0.0],
            [r[2], u[2], f[2], 0.0],
            [-vec3_dot(r, p), -vec3_dot(u, p), -vec3_dot(f, p), 1.0]
        ]
    }
}
