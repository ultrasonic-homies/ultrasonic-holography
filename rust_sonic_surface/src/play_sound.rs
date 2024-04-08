use rev1::board::Board;
use redis_async::{client, resp::FromResp};
use futures::StreamExt;


#[tokio::main]
async fn main() {
    let pubsub_con = client::pubsub_connect("127.0.0.1", 6379)
        .await
        .expect("Cannot connect to Redis");
    let mut msgs = pubsub_con
        .subscribe("music")
        .await
        .expect("Cannot subscribe to topic");
    let mut board = Board::new().unwrap();
    board.set_all_zero_phases();
    while let Some(message) = msgs.next().await {
        match message {
            Ok(message) => {
                println!("{:?}", message);
                // convert to float
                let msg = String::from_resp(message).unwrap();
                let freq: f32 = msg.parse().unwrap();
                // if freq is 0, disable, otherwise send to modulate()
                if freq == 0.0 {
                    board.modulate(0.0, false);
                } else {
                    board.modulate(freq, true);
                }
            }
            Err(e) => {
                eprintln!("ERROR: {}", e);
                break;
            }
        }
    }
    
}