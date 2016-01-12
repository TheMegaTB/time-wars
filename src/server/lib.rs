#![allow(dead_code)]
pub struct Server {
    world: Vec<u8>
}

impl Server {
    pub fn new() -> Server {
        Server {
            world: Vec::new()
        }
    }

    pub fn start_game(&self) {
        println!("A new game has been started!");
    }
}
