use chess_lib::{ChessBoard, Move, MoveFlag, Piece, Side, Square, FEN};


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

#[test]
fn insufficient_material() {
    let board = ChessBoard::from(&FEN::from("2k5/8/8/8/8/1B6/3K4/8 w - - 0 1"));
    assert!(board.is_insufficient_material());

    let board = ChessBoard::from(&FEN::from("2k5/8/8/8/8/1B3B2/3K4/8 w - - 0 1"));
    assert!(board.is_insufficient_material());

    let board = ChessBoard::from(&FEN::from("2k5/8/8/8/8/1B2B3/3K4/8 w - - 0 1"));
    assert!(!board.is_insufficient_material());

    let board = ChessBoard::from(&FEN::from("2k5/8/8/8/8/1B2N3/3K4/8 w - - 0 1"));
    assert!(!board.is_insufficient_material());

    let board = ChessBoard::from(&FEN::from("2k5/8/8/8/8/1N2N3/3K4/8 w - - 0 1"));
    assert!(!board.is_insufficient_material());

    let board = ChessBoard::from(&FEN::from("2k5/8/8/8/8/1N6/3K4/8 w - - 0 1"));
    assert!(board.is_insufficient_material());

    let board = ChessBoard::from(&FEN::from("2k5/4b3/8/8/8/1B6/3K4/8 w - - 0 1"));
    assert!(!board.is_insufficient_material());

    let board = ChessBoard::from(&FEN::from("2k5/3b4/8/8/8/1B6/3K4/8 w - - 0 1"));
    assert!(board.is_insufficient_material());
}

#[test]
fn make_move() {
    let mut board = ChessBoard::from(&FEN::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w HAha - 0 1"));
    board.make_move(Move::from_squares(Square::E2, Square::E4, MoveFlag::DOUBLE_PUSH));
    assert_eq!(board.en_passant_square(), Square::E3);
    assert_eq!(FEN::from(&board), FEN::from("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b HAha e3 0 1"));

    let mut board = ChessBoard::from(&FEN::from("5rk1/P4pp1/7p/2pP4/8/8/4PPPP/2RR1K1R w HD c6 0 1"));
    board.make_move(Move::from_squares(Square::D5, Square::C6, MoveFlag::EN_PASSANT));
    assert_eq!(FEN::from(&board), FEN::from("5rk1/P4pp1/2P4p/8/8/8/4PPPP/2RR1K1R b HD - 0 1"));

    let mut board = ChessBoard::from(&FEN::from("5rk1/P4pp1/7p/2pP4/8/8/4PPPP/2RR1K1R w HD c6 0 1"));
    board.make_move(Move::from_squares(Square::F1, Square::H1, MoveFlag::KING_SIDE_CASTLE));
    assert_eq!(FEN::from(&board), FEN::from("5rk1/P4pp1/7p/2pP4/8/8/4PPPP/2RR1RK1 b - - 1 1"));

    let mut board = ChessBoard::from(&FEN::from("5rk1/P4pp1/7p/2pP4/8/8/4PPPP/2RR1RK1 w - - 6 1"));
    board.make_move(Move::from_squares(Square::A7, Square::A8, MoveFlag::ROOK_PROMOTION));
    assert_eq!(FEN::from(&board), FEN::from("R4rk1/5pp1/7p/2pP4/8/8/4PPPP/2RR1RK1 b - - 0 1"));

    let mut board = ChessBoard::from(&FEN::from("5rk1/P4pp1/7p/2pP4/8/8/4PPPP/2RR1RK1 w - - 6 1"));
    board.make_move(Move::from_squares(Square::C1, Square::A1, MoveFlag::QUIET_MOVE));
    assert_eq!(FEN::from(&board), FEN::from("5rk1/P4pp1/7p/2pP4/8/8/4PPPP/R2R1RK1 b - - 7 1"));

    let mut board = ChessBoard::from(&FEN::from("5rk1/P4pp1/7p/3P4/2p5/8/4PPPP/R2R1K1R w HD - 0 2"));
    board.make_move(Move::from_squares(Square::F1, Square::D1, MoveFlag::QUEEN_SIDE_CASTLE));
    assert_eq!(FEN::from(&board), FEN::from("5rk1/P4pp1/7p/3P4/2p5/8/4PPPP/R1KR3R b - - 1 1"));
}