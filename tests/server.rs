mod api_test {
    extern crate server;
    use self::server::*;

    #[test]
    fn addition() {
        assert_eq!(6, plus_one(5));
    }
}
