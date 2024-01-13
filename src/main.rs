use pleco::SQ;
use pleco::{bots::RandomBot, tools::Searcher, Board, PieceType, Player};
use rand::Rng;
use std::io;
use std::io::Write;

struct GameState {
    player_white: bool,
    board: Board,
}

impl GameState {
    fn new() -> GameState {
        let mut rng = rand::thread_rng();
        let random_bool: bool = rng.gen();
        GameState {
            player_white: random_bool,
            board: Board::start_pos(),
        }
    }
}

fn parse_square(s: &str) -> SQ {
    todo!("finishme")
}

fn ask_user(prompt: &str) -> String {
    print!("{}: ", prompt);
    // io::Write::flush(&mut io::stdout());
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
    let game0 = GameState::new();
    loop {
        todo!()
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
