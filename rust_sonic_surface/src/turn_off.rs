use rev1::board::Board;

fn main() {
    let mut board = Board::new().unwrap();
    board.shut_up();
}