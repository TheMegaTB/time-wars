#![allow(dead_code)]
extern crate server as s;

pub trait API {
    fn start_game(&mut self);
}

// ----------------------------------------- LOCAL SERVER ----------------------------------------

struct LocalServer {
    server: s::Server
}

impl LocalServer {
    fn new() -> LocalServer {
        LocalServer {
            server: s::Server::new()
        }
    }
}

impl API for LocalServer {
    fn start_game(&mut self) {
        self.server.start_game();
    }
}

// ---------------------------------------- REMOTE SERVER ----------------------------------------

struct RemoteServer {
    server: s::Server
}

impl RemoteServer {
    fn new() -> RemoteServer {
        RemoteServer {
            server: s::Server::new()
        }
    }
}

impl API for RemoteServer {
    fn start_game(&mut self) {
        self.server.start_game();
    }
}

// ----------------------------------------- CONSTRUCTOR -----------------------------------------

pub struct Server {
    difficulty: i8
}

impl Server {
    pub fn new() -> Server {
        Server {
            difficulty: 0
        }
    }

    pub fn difficulty(mut self, d: i8) -> Server {
        self.difficulty = d;
        self
    }

    pub fn local(&self) -> LocalServer {
        LocalServer::new()
    }

    pub fn connect_to(&self) -> RemoteServer {
        RemoteServer::new()
    }
}
