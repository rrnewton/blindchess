use pleco::bots::{AlphaBetaSearcher, JamboreeSearcher, ParallelMiniMaxSearcher};
use pleco::{board, BitMove, File, MoveList, Piece, Rank, SQ};
use pleco::{tools::Searcher, Board, Player};
use rand::Rng;
use std::io;
use std::io::Write;

// use crate::pgn::PGNMove;

struct GameState {
    player_white: bool,
    board: Board,
    depth: u16,
    conclusion: Conclusion,
    illegalmovestrikes: i16,
}
#[derive(Copy, Clone, Debug, PartialEq)]
enum Conclusion {
    Ongoing,
    Draw,
    WhiteWin,
    BlackWin,
}
impl GameState {
    fn new() -> GameState {
        let mut rng = rand::thread_rng();
        let random_bool: bool = rng.gen();

        GameState {
            player_white: random_bool,
            board: Board::start_pos(),
            depth: ask_user("Welcome To The Blind Chess Pit Of Despair\nWhen your move is asked you can respond with\n   - A move in chess notation to continue the game\n   - \"resign\" to resign\n   - \"eval\" for eval\nWhat Difficulty Do You Want Your Opponent To Be At (2 - 7)")
                .trim()
                .parse::<u16>()
                .unwrap(),
            conclusion: Conclusion::Ongoing,
            illegalmovestrikes: 0,
        }
    }
}

// TODO: could enable a PGN syntax option
/*
fn parse_move(s: &str) -> Result<BitMove, String> {
    match PGNMove::parse(s) {
        Ok(pgnmove) => {
            // pgnmove.
            let x = pgnmove.move_type;

            todo!();
        }
        Err(e) => Err(format!("{:?}", e)),
    }
}
 */

fn parse_rank(c: char) -> Result<Rank, String> {
    match c {
        '1' => Ok(Rank::R1),
        '2' => Ok(Rank::R2),
        '3' => Ok(Rank::R3),
        '4' => Ok(Rank::R4),
        '5' => Ok(Rank::R5),
        '6' => Ok(Rank::R6),
        '7' => Ok(Rank::R7),
        '8' => Ok(Rank::R8),
        _ => Err(format!("Invalid rank character (1..8): {}", c)),
    }
}

fn parse_file(c: char) -> Result<File, String> {
    match c.to_ascii_lowercase() {
        'a' => Ok(File::A),
        'b' => Ok(File::B),
        'c' => Ok(File::C),
        'd' => Ok(File::D),
        'e' => Ok(File::E),
        'f' => Ok(File::F),
        'g' => Ok(File::G),
        'h' => Ok(File::H),
        _ => Err(format!("Invalid file character (a..h): {}", c)),
    }
}

// Take a two-element string slice such as "f3".
fn parse_filerank(s: &str) -> Result<SQ, String> {
    parse_file(s.chars().nth(0).unwrap()).and_then(|file| {
        parse_rank(s.chars().nth(1).unwrap()).and_then(|rank| Ok(SQ::make(file, rank)))
    })
}
fn srcdestnotationtobitmove(s: String, is_white: bool, board: Board) -> Result<BitMove, String> {
    parse_filerank(&s[0..2]).and_then(|src| {
        parse_filerank(&s[2..4]).and_then(|dst| {
            let mut promotag: u16 = 0;
            if ((is_white && (dst.rank() == Rank::R8)) || (!is_white && (dst.rank() == Rank::R1)))
                && ((board.piece_at_sq(src) == Piece::WhitePawn)
                    || (board.piece_at_sq(src) == Piece::BlackPawn))
            {
                let piecepromo = ask_user("What Piece Do You Want To Promote This Pawn To");
                match piecepromo.trim() {
                    "N" => {
                        if board.piece_at_sq(dst) == Piece::None {
                            promotag = 8
                        } else {
                            promotag = 12
                        }
                    }
                    "B" => {
                        if board.piece_at_sq(dst) == Piece::None {
                            promotag = 9
                        } else {
                            promotag = 13
                        }
                    }
                    "R" => {
                        if board.piece_at_sq(dst) == Piece::None {
                            promotag = 10
                        } else {
                            promotag = 14
                        }
                    }
                    "Q" => {
                        if board.piece_at_sq(dst) == Piece::None {
                            promotag = 11
                        } else {
                            promotag = 15
                        }
                    }
                    _ => return Result::Err("".to_string()),
                }
            }
            let mv = BitMove::make(promotag, src, dst);
            return Result::Ok(mv);
        })
    })
}
fn parse_move(untrimmeds: &str, board: Board, is_white: bool) -> Result<BitMove, String> {
    let mut s = untrimmeds.trim().to_string();
    let mut alllegalmoves = board.generate_moves().to_vec();
    if s.to_lowercase() == "o-o" {
        alllegalmoves = alllegalmoves
            .iter()
            .filter(|legalmove: &&BitMove| legalmove.flag() == BitMove::FLAG_KING_CASTLE)
            .map(|bm| bm.clone())
            .collect::<Vec<BitMove>>();
        if alllegalmoves.len() > 0 {
            return Result::Ok(alllegalmoves[0]);
        } else {
            return Err("".to_string());
        }
    } else if s.to_lowercase() == "o-o-o" {
        alllegalmoves = alllegalmoves
            .iter()
            .filter(|legalmove: &&BitMove| legalmove.flag() == BitMove::FLAG_QUEEN_CASTLE)
            .map(|bm| bm.clone())
            .collect::<Vec<BitMove>>();
        if alllegalmoves.len() > 0 {
            return Result::Ok(alllegalmoves[0]);
        } else {
            return Err("".to_string());
        }
    }
    if s.len() > 1 {
        s = s
            .chars()
            .into_iter()
            .filter(|c| match c {
                '1' => true,
                '2' => true,
                '3' => true,
                '4' => true,
                '5' => true,
                '6' => true,
                '7' => true,
                '8' => true,
                'a' => true,
                'b' => true,
                'c' => true,
                'd' => true,
                'e' => true,
                'f' => true,
                'g' => true,
                'h' => true,
                'A' => true,
                'B' => true,
                'C' => true,
                'D' => true,
                'E' => true,
                'F' => true,
                'G' => true,
                'H' => true,
                'N' => true,
                'R' => true,
                'Q' => true,
                'K' => true,
                'n' => true,
                'r' => true,
                'q' => true,
                'k' => true,
                _ => false,
            })
            .collect::<String>();
        let mut schars = s.chars().into_iter().collect::<Vec<char>>();
        let destindicator = &s[schars.len() - 2..schars.len()];
        alllegalmoves = alllegalmoves
            .iter()
            .filter(|legalmove: &&BitMove| {
                legalmove.to_string()[2..4].to_owned() == destindicator.to_owned().to_lowercase()
            })
            .map(|bm| bm.clone())
            .collect::<Vec<BitMove>>();
        match schars[0] {
            'N' => {
                alllegalmoves = alllegalmoves
                    .iter()
                    .filter(|legalmove: &&BitMove| {
                        ((board.piece_at_sq(legalmove.get_src()) == Piece::BlackKnight)
                            || (board.piece_at_sq(legalmove.get_src()) == Piece::WhiteKnight))
                    })
                    .map(|bm| bm.clone())
                    .collect::<Vec<BitMove>>();
                s = s[1..s.len()].to_owned();
            }
            'B' => {
                alllegalmoves = alllegalmoves
                    .iter()
                    .filter(|legalmove: &&BitMove| {
                        ((board.piece_at_sq(legalmove.get_src()) == Piece::BlackBishop)
                            || (board.piece_at_sq(legalmove.get_src()) == Piece::WhiteBishop))
                    })
                    .map(|bm| bm.clone())
                    .collect::<Vec<BitMove>>();
                s = s[1..s.len()].to_owned();
            }
            'R' => {
                alllegalmoves = alllegalmoves
                    .iter()
                    .filter(|legalmove: &&BitMove| {
                        ((board.piece_at_sq(legalmove.get_src()) == Piece::BlackRook)
                            || (board.piece_at_sq(legalmove.get_src()) == Piece::WhiteRook))
                    })
                    .map(|bm| bm.clone())
                    .collect::<Vec<BitMove>>();
                s = s[1..s.len()].to_owned();
            }
            'Q' => {
                alllegalmoves = alllegalmoves
                    .iter()
                    .filter(|legalmove: &&BitMove| {
                        ((board.piece_at_sq(legalmove.get_src()) == Piece::BlackQueen)
                            || (board.piece_at_sq(legalmove.get_src()) == Piece::WhiteQueen))
                    })
                    .map(|bm| bm.clone())
                    .collect::<Vec<BitMove>>();
                s = s[1..s.len()].to_owned();
            }
            'K' => {
                alllegalmoves = alllegalmoves
                    .iter()
                    .filter(|legalmove: &&BitMove| {
                        ((board.piece_at_sq(legalmove.get_src()) == Piece::BlackKing)
                            || (board.piece_at_sq(legalmove.get_src()) == Piece::WhiteKing))
                    })
                    .map(|bm| bm.clone())
                    .collect::<Vec<BitMove>>();
                s = s[1..s.len()].to_owned();
            }
            _ => {
                alllegalmoves = alllegalmoves
                    .iter()
                    .filter(|legalmove: &&BitMove| {
                        ((board.piece_at_sq(legalmove.get_src()) == Piece::BlackPawn)
                            || (board.piece_at_sq(legalmove.get_src()) == Piece::WhitePawn))
                    })
                    .map(|bm| bm.clone())
                    .collect::<Vec<BitMove>>();
            }
        }
        schars = s.chars().into_iter().collect::<Vec<char>>();
        if s.len() > 2 {
            alllegalmoves = alllegalmoves
                .iter()
                .filter(|legalmove: &&BitMove| {
                    legalmove.to_string()[0..1].to_owned() == schars[0].to_string().to_lowercase()
                })
                .map(|bm| bm.clone())
                .collect::<Vec<BitMove>>();
            if s.len() > 3 {
                alllegalmoves = alllegalmoves
                    .iter()
                    .filter(|legalmove: &&BitMove| {
                        // println!("{}, {}", legalmove.to_string(), schars[1].to_string());
                        legalmove.to_string()[1..2].to_string() == schars[1].to_string()
                    })
                    .map(|bm| bm.clone())
                    .collect::<Vec<BitMove>>();
            }
        }
        if alllegalmoves.len() > 0 {
            let cmove = alllegalmoves[0];
            let dst = cmove.get_dest();
            let src = cmove.get_src();
            let mut promotag: u16 = cmove.flag();
            if ((is_white && (dst.rank() == Rank::R8)) || (!is_white && (dst.rank() == Rank::R1)))
                && ((board.piece_at_sq(src) == Piece::WhitePawn)
                    || (board.piece_at_sq(src) == Piece::BlackPawn))
            {
                let piecepromo = ask_user("What Piece Do You Want To Promote This Pawn To");
                match piecepromo.trim() {
                    "N" => {
                        if board.piece_at_sq(dst) == Piece::None {
                            promotag = 8
                        } else {
                            promotag = 12
                        }
                    }
                    "B" => {
                        if board.piece_at_sq(dst) == Piece::None {
                            promotag = 9
                        } else {
                            promotag = 13
                        }
                    }
                    "R" => {
                        if board.piece_at_sq(dst) == Piece::None {
                            promotag = 10
                        } else {
                            promotag = 14
                        }
                    }
                    "Q" => {
                        if board.piece_at_sq(dst) == Piece::None {
                            promotag = 11
                        } else {
                            promotag = 15
                        }
                    }
                    _ => return Result::Err("".to_string()),
                }
            }
            let mv = BitMove::make(promotag, src, dst);
            return Result::Ok(mv);
        }
    }
    return Err("".to_string());
}

fn ask_user(prompt: &str) -> String {
    print!("{}: ", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            return input;
        }
        Err(error) => {
            panic!("Error reading from stdin: {}", error);
        }
    }
}
fn piecetolongstring(piece: Piece) -> String {
    use Piece::*;
    match piece {
        WhitePawn => "Pawn".to_string(),
        WhiteKnight => "Knight".to_string(),
        WhiteBishop => "Bishop".to_string(),
        WhiteRook => "Rook".to_string(),
        WhiteQueen => "Queen".to_string(),
        WhiteKing => "King".to_string(),
        BlackPawn => "Pawn".to_string(),
        BlackKnight => "Knight".to_string(),
        BlackBishop => "Bishop".to_string(),
        BlackRook => "Rook".to_string(),
        BlackQueen => "Queen".to_string(),
        BlackKing => "King".to_string(),
        None => "".to_string(),
    }
}
fn piecetoshortstring(piece: Piece) -> String {
    use Piece::*;
    match piece {
        WhitePawn => "".to_string(),
        WhiteKnight => "N".to_string(),
        WhiteBishop => "B".to_string(),
        WhiteRook => "R".to_string(),
        WhiteQueen => "Q".to_string(),
        WhiteKing => "K".to_string(),
        BlackPawn => "".to_string(),
        BlackKnight => "N".to_string(),
        BlackBishop => "B".to_string(),
        BlackRook => "R".to_string(),
        BlackQueen => "Q".to_string(),
        BlackKing => "K".to_string(),
        None => "".to_string(),
    }
}
fn game_loop() {
    let mut game = GameState::new();
    println!(
        "Congratulations, you are {}.",
        if game.player_white { "white" } else { "black" }
    );
    while game.conclusion == Conclusion::Ongoing {
        // println!("Current board state:\n{}", &game.board);

        if game.conclusion == Conclusion::Ongoing {
            let turn = game.board.turn();
            if (turn == Player::White) == game.player_white {
                let plrmove = ask_user("Your move");
                let mut contin = true;
                if plrmove.trim().to_lowercase() == "resign" {
                    contin = false;
                    if game.player_white {
                        game.conclusion = Conclusion::BlackWin;
                    }else{
                        game.conclusion = Conclusion::WhiteWin;
                    }
                }
                if plrmove.trim().to_lowercase() == "eval" {
                    contin = false;
                    println!("\n{:?}\n", 
                    (
                        (
                        (3 * game.board.count_piece(Player::White, pleco::PieceType::N)) + 
                        (3 * game.board.count_piece(Player::White, pleco::PieceType::B)) + 
                        (5 * game.board.count_piece(Player::White, pleco::PieceType::R)) + 
                        (9 * game.board.count_piece(Player::White, pleco::PieceType::Q)) + 
                        (1 * game.board.count_piece(Player::White, pleco::PieceType::P)) 
                        ) as f64 -
                        (
                            (3 * game.board.count_piece(Player::Black, pleco::PieceType::N)) + 
                            (3 * game.board.count_piece(Player::Black, pleco::PieceType::B)) + 
                            (5 * game.board.count_piece(Player::Black, pleco::PieceType::R)) + 
                            (9 * game.board.count_piece(Player::Black, pleco::PieceType::Q)) + 
                            (1 * game.board.count_piece(Player::Black, pleco::PieceType::P)) 
                             ) as f64
                    ));
                }
                if contin 
                {
                    let uncheckedmvhelper = parse_move(&plrmove, game.board.clone(), game.player_white);
                    if uncheckedmvhelper.is_ok() {
                        let mut uncheckedmv: BitMove = uncheckedmvhelper.clone().unwrap();
                        game.board.apply_move(uncheckedmv);
                        let youmove = game.board.last_move();
                        if youmove.is_some() {
                            // You move checked.
                            let mut youmovec: BitMove = youmove.unwrap();
                            let mut movestring = youmovec.to_string();
                            let mut boardonestatebefore = game.board.clone();
                            boardonestatebefore.undo_move();
                            let mut shortpeicemovedstring: String = piecetoshortstring(
                                boardonestatebefore.piece_at_sq(youmovec.get_src()),
                            );
                            let mut longpeicemovedstring: String = piecetolongstring(
                                boardonestatebefore.piece_at_sq(youmovec.get_src()),
                            );
                            let mut descriptivestring: String = "".to_string();
                            match youmovec.move_type() {
                                pleco::core::piece_move::MoveType::EnPassant => {
                                    descriptivestring = " with Enpassant".to_string();
                                }
                                pleco::core::piece_move::MoveType::Promotion => {
                                    descriptivestring = format!(
                                        " with promotion to {}",
                                        youmovec.promo_piece().to_string()
                                    );
                                }
                                _ => {}
                            }
                            match youmovec.move_type() {
                                pleco::core::piece_move::MoveType::Castle => {
                                    println!(
                                        "You ({}) castle {}",
                                        {
                                            if game.player_white {
                                                "White"
                                            } else {
                                                "Black"
                                            }
                                        },
                                        {
                                            if youmovec.is_king_castle() {
                                                "Kingside\n\n: O-O\n"
                                            } else {
                                                "Queenside\n\n: O-O-O\n"
                                            }
                                        }
                                    );
                                }
                                _ => {
                                    println!(
                                        "You ({}) move {} from {} to {}{}\n\n: {}{}\n",
                                        {
                                            if game.player_white {
                                                "White"
                                            } else {
                                                "Black"
                                            }
                                        },
                                        longpeicemovedstring,
                                        &movestring[0..2],
                                        &movestring[2..4],
                                        descriptivestring,
                                        shortpeicemovedstring,
                                        &movestring[2..4],
                                    );
                                }
                            }
                        }
                    } else {
                        game.illegalmovestrikes += 1;
                        if game.illegalmovestrikes == 1 {
                            println!("NOT A LEGAL MOVE, 2 MORE STRIKES BEFORE AUTO-RESIGN!",);
                        }
                        if game.illegalmovestrikes == 2 {
                            println!("NOT A LEGAL MOVE, 1 MORE STRIKE BEFORE AUTO-RESIGN!",);
                        }
                        if game.illegalmovestrikes == 3 {
                            println!("NOT A LEGAL MOVE, 3 ILLEGAL MOVES ATTEMPTED, IMMEDIATE AUTO-RESIGN!");
                            if game.player_white {
                                game.conclusion = Conclusion::BlackWin;
                            } else {
                                game.conclusion = Conclusion::WhiteWin;
                            }
                        }
                    }
                }
            } else {
                let botmove =
                    pleco::bots::AlphaBetaSearcher::best_move(game.board.clone(), game.depth);
                let mut movestring = botmove.to_string();
                let mut shortpeicemovedstring: String =
                    piecetoshortstring(game.board.piece_at_sq(botmove.get_src()));
                let mut longpeicemovedstring: String =
                    piecetolongstring(game.board.piece_at_sq(botmove.get_src()));
                let mut descriptivestring: String = "".to_string();
                match botmove.move_type() {
                    pleco::core::piece_move::MoveType::EnPassant => {
                        descriptivestring = " with Enpassant".to_string();
                    }
                    pleco::core::piece_move::MoveType::Promotion => {
                        descriptivestring =
                            format!(" with promotion to {}", botmove.promo_piece().to_string());
                    }
                    _ => {}
                }
                match botmove.move_type() {
                    pleco::core::piece_move::MoveType::Castle => {
                        println!(
                            "Your opponent ({}) castles {}",
                            {
                                if game.player_white {
                                    "Black"
                                } else {
                                    "White"
                                }
                            },
                            {
                                if botmove.is_king_castle() {
                                    "Kingside\n\n: O-O\n"
                                } else {
                                    "Queenside\n\n: O-O-O\n"
                                }
                            }
                        );
                    }
                    _ => {
                        println!(
                            "Your opponent ({}) moves {} from {} to {}{}\n\n: {}{}\n",
                            {
                                if game.player_white {
                                    "Black"
                                } else {
                                    "White"
                                }
                            },
                            longpeicemovedstring,
                            &movestring[0..2],
                            &movestring[2..4],
                            descriptivestring,
                            shortpeicemovedstring,
                            &movestring[2..4],
                        );
                    }
                }
                game.board.apply_move(botmove);
            }
            if game.board.checkmate() {
                if turn == Player::White {
                    game.conclusion = Conclusion::WhiteWin;
                } else {
                    game.conclusion = Conclusion::BlackWin;
                }
            }
            if game.board.stalemate() {
                game.conclusion = Conclusion::Draw;
            }
            if game.board.rule_50() >= 49 {
                game.conclusion = Conclusion::Draw;
            }
        }
    }
    println!("\nCurrent board state at end of game:\n{}", &game.board);
    println!("Game ended in a {}", {
        match game.conclusion {
            Conclusion::Draw => "Draw".to_string(),
            Conclusion::Ongoing => "NOTHING".to_string(),
            Conclusion::WhiteWin => {
                if game.player_white {
                    format!("Win! You beat level {}.", game.depth)
                } else {
                    format!("Loss against level {}.", game.depth)
                }
            }
            Conclusion::BlackWin => {
                if !game.player_white {
                    format!("Win! You beat level {}", game.depth)
                } else {
                    format!("Loss against level {}.", game.depth)
                }
            }
        }
    });
}

fn main() {
    // let board = Board::start_pos();
    // assert_eq!(board.count_piece(Player::White, PieceType::P), 8);
    // assert_eq!(
    //     &board.fen(),
    //     "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    // );

    // let nxtmove = RandomBot::best_move(board, 3);
    // println!("Move {}", nxtmove);

    // let answer = ask_user("Your name");
    // println!("Done, got response <{}>", answer);
    game_loop();
}
