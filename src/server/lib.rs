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
    Knight,
    Builder
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

    fn get_ai(&self, id: ID) -> &AI {
        &self.ais[id]
    }

    fn closest_keyframe(&self, target: TimeIndex) -> (usize, Keyframe) {
        let x = self.keyframes.iter().rev().find(|&(key, _)| *key <= target).unwrap();
        (*x.0, x.1.clone())
    }

    fn calculate(&mut self, target: TimeIndex) -> Keyframe { //TODO: Don't use .clone() all over the place and use pointers instead
        let closest = self.closest_keyframe(target);
        if closest.0 == target { return closest.1 };
        println!("{}", closest.0);

        let mut current: TimeIndex = closest.0;
        let mut last = closest.1;
        while current != target {
            current = current + 1;
            println!("Target: {} | Current: {}", target, current);
            let mut ais = Vec::with_capacity(last.len());
            for x in last.iter() {
                let id = x.0;
                let mut loc = x.1;
                let o = x.2;
                let ai = self.get_ai(id);

                if ai.ai_type == AIType::Builder {
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

    pub fn start_game(&mut self) {
        println!("A new game has been started!");

        // ------------------------------- TEST CODE -------------------------------
        let ai = AI {
            ai_type: AIType::Builder,
            player: 0,
            start_location: (0.0, 0.0),
            start_orientation: 0.0
        };
        self.ais.push(ai);
        self.keyframes.insert(0, vec![(0, (0.0, 0.0), 0.0)]);

        self.calculate(10);
        self.print_keyframes();
        println!("-----------------------------");
        self.calculate(20);
        self.print_keyframes();
        // -------------------------------------------------------------------------
    }
}
