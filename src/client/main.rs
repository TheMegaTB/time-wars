mod networking;
use networking::API;
use networking::Server;

fn main() {
    let mut server = Server::new().difficulty(5).local();
    server.start_game();
}
