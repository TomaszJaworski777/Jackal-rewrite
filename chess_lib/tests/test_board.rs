use chess_lib::{ChessBoard, Piece, Side, Square, FEN};


#[test]
fn from_fen() {
    let board = ChessBoard::from(&FEN::start_position());

    assert_eq!(board.get_king_square(Side::WHITE), Square::E1);
    assert_eq!(board.get_piece_mask_for_side(Piece::PAWN, Side::WHITE).ls1b_square(), Square::A2);
    assert_eq!(board.get_piece_on_square(Square::B1), Piece::KNIGHT);

    assert_eq!(FEN::from(&board), FEN::start_position());

    let board = ChessBoard::from(&FEN::kiwipete_position());
    assert_eq!(FEN::from(&board), FEN::kiwipete_position());

    let fen = FEN::from("rrkrrrrr/pp3ppp/3pp3/2p5/5P2/P2P4/1PP1P1PP/RRRRKRRR w FBda - 2 1");
    assert_eq!(FEN::from(&ChessBoard::from(&fen)), fen);

    let fen = FEN::from("brkr2rr/pp3ppp/3ppn2/2p5/5P2/P2P4/NPP1P1PP/BQRBKR1R w HChb - 2 1");
    assert_eq!(FEN::from(&ChessBoard::from(&fen)), fen);

    let fen = FEN::from("brnr1krr/pp3ppp/3ppn2/2p5/5P2/P2P4/NPP1P1PP/BQ1BRRKR w HEgb - 2 1");
    assert_eq!(FEN::from(&ChessBoard::from(&fen)), fen);
}
