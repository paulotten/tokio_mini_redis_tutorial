// https://tokio.rs/tokio/tutorial/streams#iteration

use tokio::stream::StreamExt;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (mut tx, mut rx) = mpsc::channel(10);

    tokio::spawn(async move {
        tx.send(1).await.unwrap();
        tx.send(2).await.unwrap();
        tx.send(3).await.unwrap();
    });

    while let Some(v) = rx.next().await {
        println!("GOT = {:?}", v);
    }
}
