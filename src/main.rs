use pleco::bots::AlphaBetaSearcher;
use pleco::{tools::Searcher, Board, Player};
use pleco::{BitMove, File, Rank, SQ};
use rand::Rng;
use std::io;
use std::io::Write;

// use crate::pgn::PGNMove;

pub mod pgn;
struct GameState {
    player_white: bool,
    board: Board,
    depth: u16,
}

impl GameState {
    fn new() -> GameState {
        let mut rng = rand::thread_rng();
        let random_bool: bool = rng.gen();
        GameState {
            player_white: random_bool,
            board: Board::start_pos(),
            depth: 4,
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

fn parse_move(s: &str) -> Result<BitMove, String> {
    if s.len() != 4 {
        return Err("Move string not of length four!".to_owned());
    }
    parse_filerank(&s[0..2]).and_then(|src| {
        parse_filerank(&s[2..4]).and_then(|dst| {
            let flag_bits = todo!("Make the BitMove...");
            let mv = BitMove::make(flag_bits, src, dst);
            todo!()
        })
    })
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

fn game_loop() {
    let mut game = GameState::new();
    println!(
        "Congratulations, you are {}.",
        if game.player_white { "white" } else { "black" }
    );
    loop {
        println!("Current board state:\n{}", &game.board);

        let turn = game.board.turn();
        if (turn == Player::White) == game.player_white {
            println!("Your move");
            let plrmove = ask_user("Your move");
            if let Ok(mv) = parse_move(&plrmove) {
                game.board.apply_move(mv);
            } else {
                // TODO change to match and print the error...
                // println!("")
            }
        } else {
            let botmove = AlphaBetaSearcher::best_move(game.board.clone(), game.depth);
            println!("Bot moves {}\n", botmove);
            game.board.apply_move(botmove);
        }
    }
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
