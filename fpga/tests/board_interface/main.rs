use rev1::board::Board;

fn main() {
    match Board::new() {
        Ok(mut board) => {
            let phases: Vec<f32> = vec![0.00, 0.79, 1.57, 2.36, 3.14, 3.97, 4.71, 5.49];
            board.set_frame(&phases);
            let phases: Vec<f32> = vec![-0.1, 6.0, 7.0, 3.97, 4.71, 5.49];
            board.set_frame(&phases);
        }
        Err(error) => {
            println!("{}", error);
        }
    }
}
