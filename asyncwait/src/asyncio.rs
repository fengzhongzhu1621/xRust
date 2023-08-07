use async_stream;

use futures_lite::AsyncReadExt;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

pub fn stream() {
    futures_lite::future::block_on(async {
        // 使用stream实现了 yield
        let s = async_stream::stream! {
            for i in 0..3 {
                yield i;
            }
        };

        // 创建一个 Pin<&mut T>， 并将 AsyncStream 其固定在栈上
        pin_mut!(s); // needed for iteration

        // 打印 stream
        while let Some(value) = s.next().await {
            println!("stream got {}", value);
        }
    });
}

pub fn futures_lite_io() {
    futures_lite::future::block_on(async {
        let input: &[u8] = b"hello";
        let mut reader = futures_lite::io::BufReader::new(input);

        let mut contents = String::new();
        reader.read_to_string(&mut contents).await.unwrap();

        println!("futures_lite_io got {}", contents);
    });
}
