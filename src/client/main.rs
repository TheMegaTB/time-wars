// ------------------- GFX -------------------
#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
use gfx::traits::*;

// ------------------- Piston -------------------
extern crate piston_window;
extern crate piston;
extern crate sdl2_window;
extern crate camera_controllers;
use piston_window::*;
use piston::input;
use sdl2_window::Sdl2Window;
use camera_controllers::{
    FirstPersonSettings,
    FirstPerson,
    CameraPerspective,
    model_view_projection
};
// ------------------- Other -------------------
extern crate vecmath;
extern crate rand;
extern crate time;
extern crate find_folder;

// ------------------- Intern -------------------
mod world;
mod math_fx;
mod gfx_lib;
mod consts;
use world::*;
use gfx_lib::*;

// ------------------- Network -------------------
mod networking;
use networking::API;
use networking::Server;

fn main() {
    let mut server = Server::new().difficulty(5).local();
    server.start_game();

    let mut events: PistonWindow<(), Sdl2Window> =
        WindowSettings::new("Timewars", [640, 480])
        .exit_on_esc(true)
        .samples(4)
        .build()
        .unwrap();
    events.set_capture_cursor(true);

    let ref mut factory = events.factory.borrow().clone();

    let mut my_world = world::World::example();

    let assets_path = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    my_world.load_animations(assets_path.as_path().to_str().unwrap().to_string() , 2, factory);

    let texture = factory.create_texture_rgba8_static(1, 1, &[0x00_C0_A0_20]).unwrap();

    let sampler = factory.create_sampler(
        gfx::tex::SamplerInfo::new(gfx::tex::FilterMethod::Bilinear,
                                   gfx::tex::WrapMode::Clamp)
    );

    let program = {
        let vertex = gfx::ShaderSource {
            glsl_120: Some(include_bytes!("../../assets/shader/cube_120.glslv")),
            glsl_150: Some(include_bytes!("../../assets/shader/cube_150.glslv")),
            .. gfx::ShaderSource::empty()
        };
        let fragment = gfx::ShaderSource {
            glsl_120: Some(include_bytes!("../../assets/shader/cube_120.glslf")),
            glsl_150: Some(include_bytes!("../../assets/shader/cube_150.glslf")),
            .. gfx::ShaderSource::empty()
        };
        factory.link_program_source(vertex, fragment).unwrap()
    };

    let mut data = Params {
        u_model_view_proj: vecmath::mat4_id(),
        t_color: (texture, Some(sampler)),
        _r: std::marker::PhantomData,
    };

    let get_projection = |w: &PistonWindow<(), Sdl2Window>| {
        let draw_size = w.window.borrow().draw_size();
        CameraPerspective {
            fov: 90.0, near_clip: 0.1, far_clip: 1000.0,
            aspect_ratio: (draw_size.width as f32) / (draw_size.height as f32)
        }.projection()
    };


    let mut projection = get_projection(&events);
    let mut first_person = FirstPerson::new(
        [0.5, 0.5, 4.0],
        FirstPersonSettings::keyboard_wasd()
    );

    let state = gfx::DrawState::new().depth(gfx::state::Comparison::LessEqual, true);


    for e in events {
        first_person.event(&e);

        e.press(|key| {
            // let i: i8 = key;
            match key {
                input::Button::Keyboard(k) => {
                    match k {
                        input::Key::W => my_world.player.move_player(0, 1.3),
                        input::Key::S => my_world.player.move_player(1, 1.3),
                        input::Key::A => my_world.player.move_player(2, 1.3),
                        input::Key::D => my_world.player.move_player(3, 1.3),
                        input::Key::Space => my_world.player.move_player(4, 5.0),
                        //input::Key::LShift => my_world.player.move_player(5, 1.0),
                        _ => {}
                    }
                }
                _ => {}
            }
        });

        e.release(|key| {
            // let i: i8 = key;
            match key {
                input::Button::Keyboard(k) => {
                    match k {
                        input::Key::W => my_world.player.move_player(0, 0.0),
                        input::Key::S => my_world.player.move_player(1, 0.0),
                        input::Key::A => my_world.player.move_player(2, 0.0),
                        input::Key::D => my_world.player.move_player(3, 0.0),
                        input::Key::Space => my_world.player.move_player(4, 0.0),
                        //input::Key::LShift => my_world.player.move_player(5, 0.0),
                        _ => {}
                    }
                }
                _ => {}
            }
        });

        e.mouse_relative(|x, y| {
            my_world.player.rotate_player(x as f32/150f32, y as f32/150f32);
        });

        e.draw_3d(|stream| {
            //let a = TimeDiff::start();

            // let args = e.render_args().unwrap();
            stream.clear(
                gfx::ClearData {
                    color: [0.3, 0.3, 0.3, 1.0],
                    depth: 1.0,
                    stencil: 0,
                }
            );

            let view = my_world.get_view_matrix();

            for (mesh, slice, model) in my_world.update().iter().cloned() {
                data.u_model_view_proj = model_view_projection(
                    model,
                    view,
                    projection
                );

                stream.draw(&(mesh, slice, &program, &data, &state)).unwrap();
            }
            //a.end();

            let mut vertex_data = Vec::new();
            let world_size = 100.0;
            vertex_data.push(Vertex::new(world_size, 0.0, -world_size, [0.521568627, 0.737254902, 0.274509804, 1.0]));
            vertex_data.push(Vertex::new(-world_size, 0.0, -world_size, [0.521568627, 0.737254902, 0.274509804, 1.0]));
            vertex_data.push(Vertex::new(-world_size, 0.0, world_size, [0.521568627, 0.737254902, 0.274509804, 1.0]));
            vertex_data.push(Vertex::new(world_size, 0.0, world_size, [0.521568627, 0.737254902, 0.274509804, 1.0]));

            let mesh = factory.create_mesh(&vertex_data);
            let slice = [0u8, 1, 2, 2, 3, 0].to_slice(factory, gfx::PrimitiveType::TriangleList);

            let model = vecmath::mat4_id();

            data.u_model_view_proj = model_view_projection(
                model,
                view,
                projection
            );

            stream.draw(&(&mesh, slice, &program, &data, &state)).unwrap();

        });

        if let Some(_) = e.resize_args() {
            projection = get_projection(&e);
        }
    }
}
