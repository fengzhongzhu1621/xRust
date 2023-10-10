use barrage;

pub fn barrage_example() {
    smol::block_on(async {
        let (tx, rx) = barrage::unbounded();
        let rx2 = rx.clone();
        tx.send_async("Hello!").await.unwrap();
        assert_eq!(rx.recv_async().await, Ok("Hello!"));
        assert_eq!(rx2.recv_async().await, Ok("Hello!"));
    });
}
