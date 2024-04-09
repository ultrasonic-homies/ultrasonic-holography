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
    board.set_preset_calibration();
    board.calibrate();
    let phases = vec![1.2026409, 4.957826, 1.9880395, 4.835108, 0.88357306, 2.675263, 3.8779035, 4.51604, 4.540583, 3.9760783, 2.822525, 1.0799227, 5.0805445, 2.2825637, 5.301438, 1.5953401, 4.957826, 2.552544, 5.988661, 2.6507192, 5.0560007, 0.63813615, 1.9144081, 2.552544, 2.5770879, 1.9880395, 0.7853985, 5.276894, 2.8961558, 2.3841858e-7, 2.8961558, 5.350525, 1.9880395, 5.988661, 3.2397676, 2.3841858e-7, 2.503457, 4.417865, 5.743224, 0.14726233, 0.1718061, 5.841399, 4.5896707, 2.6998067, 0.2699809, 3.5588355, 0.049087524, 2.405282, 4.835108, 2.6507192, 2.3841858e-7, 3.1170492, 5.7186804, 1.4235343, 2.7734375, 3.4852045, 3.5097482, 2.8716125, 1.5707964, 5.939574, 3.4115734, 0.3190682, 3.0188746, 5.2523503, 0.88357306, 5.0560007, 2.503457, 5.7186804, 2.086214, 4.1233406, 5.546875, 6.2586417, 0.024544, 5.6450496, 4.2951465, 2.3071074, 5.988661, 2.822525, 5.4487, 1.3008159, 2.675263, 0.63813615, 4.417865, 1.4235343, 4.1233406, 6.2340984, 1.3989905, 2.1353016, 2.184389, 1.4971656, 0.12271881, 4.3687773, 1.7180585, 4.7614765, 1.0308352, 3.0925055, 3.8779035, 1.9144081, 5.743224, 2.7734375, 5.546875, 1.3989905, 2.8716125, 3.6324666, 3.6570103, 2.9697871, 1.5707964, 5.767768, 3.0679617, 6.086836, 2.3071074, 4.3196898, 4.51604, 2.552544, 0.14726233, 3.4852045, 6.2586417, 2.1353016, 3.6324666, 4.417865, 4.4424086, 3.7306414, 2.3071074, 0.22089338, 3.779729, 0.49087405, 2.9452434, 4.957826, 4.540583, 2.5770879, 0.1718061, 3.5097482, 0.024544, 2.184389, 3.6570103, 4.4424086, 4.491496, 3.779729, 2.356195, 0.24543715, 3.8288162, 0.5154178, 2.994331, 4.98237, 3.9760783, 1.9880395, 5.841399, 2.8716125, 5.6450496, 1.4971656, 2.9697871, 3.7306414, 3.779729, 3.0679617, 1.6689714, 5.865943, 3.1661365, 6.185011, 2.3807383, 4.417865, 2.822525, 0.7853985, 4.5896707, 1.5707964, 4.2951465, 0.12271881, 1.5707964, 2.3071074, 2.356195, 1.6689714, 0.29452467, 4.540583, 1.865321, 4.9332824, 1.1780977, 3.2397676, 1.0799227, 5.276894, 2.6998067, 5.939574, 2.3071074, 4.3687773, 5.767768, 0.22089338, 0.24543715, 5.865943, 4.540583, 2.5280008, 6.2095547, 3.0434184, 5.6450496, 1.4971656, 5.0805445, 2.8961558, 0.2699809, 3.4115734, 5.988661, 1.7180585, 3.0679617, 3.779729, 3.8288162, 3.1661365, 1.865321, 6.2095547, 3.6815538, 0.58904886, 3.2643113, 5.4977875, 2.2825637, 2.3841858e-7, 3.5588355, 0.3190682, 2.822525, 4.7614765, 6.086836, 0.49087405, 0.5154178, 6.185011, 4.9332824, 3.0434184, 0.58904886, 3.8779035, 0.36815572, 2.6998067, 5.301438, 2.8961558, 0.049087524, 3.0188746, 5.4487, 1.0308352, 2.3071074, 2.9452434, 2.994331, 2.3807383, 1.1780977, 5.6450496, 3.2643113, 0.36815572, 3.2643113, 5.6941366, 1.5953401, 5.350525, 2.405282, 5.2523503, 1.3008159, 3.0925055, 4.3196898, 4.957826, 4.98237, 4.417865, 3.2397676, 1.4971656, 5.4977875, 2.6998067, 5.6941366, 1.9880395];
    board.set_frame(&phases);
    board.modulate(200.0, false);
    println!("Ready for midi");
    while let Some(message) = msgs.next().await {
        match message {
            Ok(message) => {
                // println!("{:?}", message);
                //message is a string in the form "261.53,off", split on comma and turn frist element into frequency and second into bool
                let msg = String::from_resp(message).unwrap();
                let split_msg: Vec<&str> = msg.split(",").collect();
                let freq: f32 = split_msg[0].parse().unwrap();
                let on_off: bool = split_msg[1].parse().unwrap();
                println!("Received message: {:?}", split_msg);
                board.modulate_two_boards(freq, on_off)
            }
            Err(e) => {
                eprintln!("ERROR: {}", e);
                break;
            }
        }
    }
    
}