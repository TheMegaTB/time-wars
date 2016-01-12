mod networking;
use networking::API;
use networking::Server;

fn main() {
    let server = Server::new().difficulty(5).start_local();
    server.start_game();
}
