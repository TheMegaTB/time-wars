mod api_test {
    extern crate server;
    use self::server::*;

    #[test]
    fn start_game() {
        let mut s = Server::new();
        s.start_game();
        //assert_eq!(6, plus_one(5));
    }
}
