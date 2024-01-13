use pleco::{Board,Player,PieceType};

fn main() {
    println!("Hello, world!");

    let board = Board::start_pos();
    assert_eq!(board.count_piece(Player::White,PieceType::P), 8);
    assert_eq!(&board.fen(),"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

    println!("Done.");
}
