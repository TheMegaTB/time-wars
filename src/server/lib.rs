#![allow(dead_code)]
#[cfg(feature = "f64-precision")]
type Coordinate = (f64, f64);
#[cfg(not(feature = "f64-precision"))]
type Coordinate = (f32, f32);
type TimeIndex = u32;
type ID = u16;

enum Player {
    Red,
    Blue
}

// ------------------------------------------ PORTAL -----------------------------------------

struct Endpoint {
    location: Coordinate,
    creation: TimeIndex,
    expiration: TimeIndex,
    scale: f32
}

struct Portal {
    player: Player,
    origin: Endpoint,
    dest: Endpoint,
    compression_factor: (f32, f32) // (size, time) - compression level when traveling origin->dest
}

// -------------------------------------------- AI -------------------------------------------

trait AI {
    fn calculate(t: TimeIndex) -> Coordinate;
}

pub struct BasicAI {
    difficulty: u8
}

impl AI for BasicAI {
    fn calculate(t: TimeIndex) -> Coordinate {
        (t as f32, (t as f32)/2.0)
    }
}

// ------------------------------------------ SERVER -----------------------------------------

pub struct Server {
    portals: Vec<Portal>
}

impl Server {
    pub fn new() -> Server {
        if cfg!(feature = "f64-precision") { println!("Using double precision floating mode!") }
        Server {
            portals: Vec::new(),
        }
    }

    pub fn start_game(&mut self) {
        println!("A new game has been started!");
    }
}
