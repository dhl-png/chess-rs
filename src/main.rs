use chess::Board;
use chess::Piece;
use chess::Color;
use chess::PieceType;
use chess::Position;
use chess::GameError;

fn main() ->  Result<(),GameError> {
    let mut board = Board::new();
    let pawn = Piece::new(PieceType::Pawn,Color::White);
    let rook = Piece::new(PieceType::Rook, Color::Black);
    board.tiles[0][7].insert(Some(rook));
    board.tiles[0][0].insert(Some(rook));
    for i in 0..8 {
        board.tiles[1][i].insert(Some(pawn))
    }
    println!("{}", board);
    let start_pos = Position::new(1,3)?;
    let end_pos = Position::new(1,4)?;
    board.move_piece(start_pos, end_pos)?;
    println!("{}",board);
    Ok(())
}


