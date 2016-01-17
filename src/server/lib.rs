#![allow(dead_code)]
use std::collections::BTreeMap;

#[cfg(feature = "f64-precision")]
type Coordinate = (f64, f64);
#[cfg(not(feature = "f64-precision"))]
type Coordinates = (f32, f32);
type Orientation = f32;
type Keyframe = Vec<(ID, Coordinates, Orientation)>;
type TimeIndex = usize;
type Player = usize;
type ID = usize;


// ------------------------------------------ PORTAL -----------------------------------------

struct Endpoint {
    location: Coordinates,
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

#[derive(PartialEq)]
enum AIType {
    Scout,
    Knight
}

struct AI {
    ai_type: AIType,
    player: Player,
    start_location: Coordinates,
    start_orientation: Orientation
}

// ------------------------------------------ SERVER -----------------------------------------

pub struct Server {
    portals: Vec<Portal>,
    keyframes: BTreeMap<TimeIndex, Keyframe>,
    ais: Vec<AI>
}

impl Server {
    pub fn new() -> Server {
        if cfg!(feature = "f64-precision") { println!("Using double precision floating mode!") }
        Server {
            portals: Vec::new(),
            keyframes: BTreeMap::new(),
            ais: Vec::new()
        }
    }

    fn create_portal(&mut self, player: Player,
                origin: (TimeIndex, Coordinates), origin_lifetime: TimeIndex, origin_scale: f32,
                dest: (TimeIndex, Coordinates), dest_lifetime: TimeIndex, dest_scale: f32) {
        let origin = Endpoint {
            location: origin.1,
            creation: origin.0,
            expiration: origin.0 + origin_lifetime,
            scale: origin_scale
        };
        let dest = Endpoint {
            location: dest.1,
            creation: dest.0,
            expiration: dest.0 + dest_lifetime,
            scale: dest_scale
        };
        self.portals.push(
            Portal {
                player: player,
                origin: origin,
                dest: dest,
                compression_factor: (dest_scale / origin_scale, (dest_lifetime as f32) / (origin_lifetime as f32))
            }
        )
    }

    fn get_ai(&self, id: ID) -> &AI {
        &self.ais[id]
    }

    fn get_closest_keyframe(&self, target: TimeIndex) -> (usize, Keyframe) {
        let x = self.keyframes.iter().rev().find(|&(key, _)| *key <= target).unwrap();
        (*x.0, x.1.clone())
    }

    fn calculate(&mut self, target: TimeIndex) -> Keyframe { //TODO: Don't use .clone() all over the place and use pointers instead
        let closest = self.get_closest_keyframe(target);
        if closest.0 == target { return closest.1 };

        let mut current: TimeIndex = closest.0;
        let mut last = closest.1;
        while current != target {
            current = current + 1;
            let mut ais = Vec::with_capacity(last.len());
            for x in last.iter() {
                let id = x.0;
                let mut loc = x.1;
                let o = x.2;
                let ai = self.get_ai(id);

                if ai.ai_type == AIType::Scout {
                    loc.0 = loc.0 + 1.0;
                    loc.1 = loc.0.powf(2.0);
                }

                ais.push((id, loc, o));
            }
            self.keyframes.insert(current, ais.clone());
            last = ais;
        }
        self.keyframes.get(&target).unwrap().clone()
    }

    fn print_keyframes(&self) {
        for keyframe in self.keyframes.iter() {
            for ai in keyframe.1.iter() {
                println!("AI: {}, X: {}, Y: {}", ai.0, (ai.1).0, (ai.1).1);
            }
        }
    }

    fn print_portal(&self, id: usize) {
        let ref p = self.portals[id];
        let ref origin = p.origin;
        let ref dest = p.dest;
        println!("ORIGIN: [X: {}, Y: {}, T: {}], Expiration: {}",
                    origin.location.0, origin.location.1, origin.creation, origin.expiration);
        println!("DESTIN: [X: {}, Y: {}, T: {}], Expiration: {}",
                    dest.location.0, dest.location.1, dest.creation, dest.expiration);
        println!("Compression ratio (size): {}", p.compression_factor.0);
        println!("Compression ratio (time): {}", p.compression_factor.1);
    }

    pub fn start_game(&mut self) {
        println!("A new game has been started!");
    }
}

#[test]
fn calculate_keyframes() {
    let mut s = Server::new();
    let ai = AI {
        ai_type: AIType::Scout,
        player: 0,
        start_location: (0.0, 0.0),
        start_orientation: 0.0
    };
    s.ais.push(ai);
    s.keyframes.insert(0, vec![(0, (0.0, 0.0), 0.0)]);

    s.calculate(10);
    {
        let loc = s.keyframes.get(&(10 as usize)).unwrap()[0].1;
        assert_eq!(loc, (10.0, 100.0));
    }
    s.calculate(20);
    {
        let loc = s.keyframes.get(&(20 as usize)).unwrap()[0].1;
        assert_eq!(loc, (20.0, 400.0));
    }
}

#[test]
fn compression_ratio() {
    let mut s = Server::new();
    s.create_portal(0, (100, (1.0, 1.0)), 500, 1.0, (0, (2.0, 2.0)), 100, 4.0);
    assert_eq!(s.portals[0].compression_factor.0, 4f32);
    assert_eq!(s.portals[0].compression_factor.1, 0.2f32);
}
