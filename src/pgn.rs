//! Module for generating chess boards from PGN notation.

//use super::Board;
use pleco::core::sq::SQ;
use pleco::core::{File, PieceType, Rank};
use std::fmt;

//[Event "F/S Return Match"]
//[Site "Belgrade, Serbia JUG"]
//[Date "1992.11.04"]
//[Round "29"]
//[White "Fischer, Robert J."]
//[Black "Spassky, Boris V."]
//[Result "1/2-1/2"]

// 1. e4 e5 2. Nf3 Nc6 3. Bb5 a6 4. Ba4 Nf6 5. O-O Be7 6. Re1 b5 7. Bb3 d6 8. c3
// O-O 9. h3 Nb8 10. d4 Nbd7 11. c4 c6 12. cxb5 axb5 13. Nc3 Bb7 14. Bg5 b4 15.
// Nb1 h6 16. Bh4 c5 17. dxe5 Nxe4 18. Bxe7 Qxe7 19. exd6 Qf6 20. Nbd2 Nxd6 21.
// Nc4 Nxc4 22. Bxc4 Nb6 23. Ne5 Rae8 24. Bxf7+ Rxf7 25. Nxf7 Rxe1+ 26. Qxe1 Kxf7
// 27. Qe3 Qg5 28. Qxg5 hxg5 29. b3 Ke6 30. a3 Kd6 31. axb4 cxb4 32. Ra5 Nd5 33.
// f3 Bc8 34. Kf2 Bf5 35. Ra7 g6 36. Ra6+ Kc5 37. Ke1 Nf4 38. g3 Nxh3 39. Kd2 Kb5
// 40. Rd6 Kc5 41. Ra6 Nf2 42. g4 Bd3 43. Re6 1/2-1/2

// https://www.chessclub.com/user/help/PGN-spec

#[derive(Debug)]
pub enum GameResult {
    WhiteWins,
    BlackWins,
    Draw,
    Other,
}

#[derive(Debug)]
pub enum ChessDate {
    Unknown,
    Year(u16),
    YearMonth(u16, u8),
    Full(u16, u8, u8),
}

impl ChessDate {
    pub fn parse_chess_date(date: &str) -> Self {
        let mut args = date[1..(date.len() - 1)].split('.');

        let y = args.next().map(|m: &str| m.parse::<u16>());

        if y.is_none() {
            return ChessDate::Unknown;
        }
        let y_err = y.unwrap();
        if y_err.is_err() {
            return ChessDate::Unknown;
        }
        let year = y_err.unwrap();

        let m = args.next().map(|m: &str| m.parse::<u8>());

        if m.is_none() {
            return ChessDate::Year(year);
        }
        let m_err = m.unwrap();
        if m_err.is_err() {
            return ChessDate::Year(year);
        }
        let month = m_err.unwrap();

        let d = args.next().map(|m: &str| m.parse::<u8>());

        if d.is_none() {
            return ChessDate::YearMonth(year, month);
        }

        let d_err = d.unwrap();
        if d_err.is_err() {
            return ChessDate::YearMonth(year, month);
        }
        let day = d_err.unwrap();
        ChessDate::Full(year, month, day)
    }

    pub fn to_string(&self) -> String {
        match *self {
            ChessDate::Unknown => ("\"??\"").to_owned(),
            ChessDate::Year(y) => {
                let mut s = ("\"").to_owned();
                s.push_str(y.to_string().as_ref());
                s.push_str(".??.??\"");
                s
            }
            ChessDate::YearMonth(y, m) => {
                let mut s = ("\"").to_owned();
                s.push_str(y.to_string().as_ref());
                s.push('.');
                s.push_str(m.to_string().as_ref());
                s.push_str(".??\"");
                s
            }
            ChessDate::Full(y, m, d) => {
                let mut s = ("\"").to_owned();
                s.push_str(y.to_string().as_ref());
                s.push('.');
                s.push_str(m.to_string().as_ref());
                s.push('.');
                s.push_str(d.to_string().as_ref());
                s.push('"');
                s
            }
        }
    }
}

#[derive(Debug)]
pub struct ChessRound {
    rounds: Vec<u32>,
}

impl Default for ChessRound {
    fn default() -> ChessRound {
        ChessRound { rounds: Vec::new() }
    }
}

impl ChessRound {
    pub fn parse_chess_round(round: &str) -> ChessRound {
        let mut cr = ChessRound::default();
        let args = round[1..(round.len() - 1)].split('.');
        args.for_each(|r: &str| {
            //            r.parse().map(|m: u32| cr.rounds.push(m));
            if let Ok(m) = r.parse() {
                cr.rounds.push(m)
            }
        });
        cr
    }

    pub fn to_string(&self) -> String {
        let mut s = "\"".to_string();
        for (i, x) in self.rounds.iter().enumerate() {
            s.push_str(x.to_string().as_ref());
            if i != self.rounds.len() - 1 {
                s.push('.');
            }
        }
        s.push('"');
        s
    }
}

#[derive(Debug)]
pub struct PGNTags {
    event: String,
    site: String,
    date: ChessDate,
    round: ChessRound,
    white: String,
    black: String,
    result: String,
}

impl fmt::Display for PGNTags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.to_string();
        f.pad(&s)
    }
}

impl PGNTags {
    pub fn to_string(&self) -> String {
        let mut s: String = "[Event ".to_owned();
        s.push_str(self.event.as_ref());
        s.push_str("]\n[Site ");
        s.push_str(self.site.as_ref());
        s.push_str("]\n[Date ");
        s.push_str(self.date.to_string().as_ref());
        s.push_str("]\n[Round ");
        s.push_str(self.round.to_string().as_ref());
        s.push_str("]\n[White ");
        s.push_str(self.white.as_ref());
        s.push_str("]\n[Black ");
        s.push_str(self.black.as_ref());
        s.push_str("]\n[Result ");
        s.push_str(self.result.as_ref());
        s.push_str("]\n");
        s
    }

    pub fn add(mut self, input: &str) -> Result<PGNTags, PGNError> {
        let first_char = input.chars().nth(0).ok_or(PGNError::TagParse)?;
        let last_char = input.chars().last().ok_or(PGNError::TagParse)?;
        if input.len() < 3 || first_char != '[' || last_char != ']' {
            return Err(PGNError::TagParse);
        }
        let r = &input[1..(input.len() - 1)];
        let quote_first = r.find('"').ok_or(PGNError::TagParse)?;
        let quote_second = r.rfind('"').ok_or(PGNError::TagParse)?;
        if quote_first >= quote_second - 2 {
            return Err(PGNError::TagParse);
        }
        let in_quote = r[(quote_first)..(quote_second + 1)].to_owned();
        let no_quote = r[..(quote_first - 1)]
            .trim()
            .split_whitespace()
            .next()
            .ok_or(PGNError::TagParse)?;
        self = self.parse_tag(no_quote, in_quote)?;
        Ok(self)
    }

    pub fn parse_tag(mut self, tag: &str, data: String) -> Result<PGNTags, PGNError> {
        match tag {
            "Event" => self.event = data,
            "Site" => self.site = data,
            "Date" => self.date = ChessDate::parse_chess_date(data.as_ref()),
            "Round" => self.round = ChessRound::parse_chess_round(data.as_ref()),
            "White" => self.white = data,
            "Black" => self.black = data,
            "Result" => self.result = data,
            _ => return Err(PGNError::TagParse),
        }
        Ok(self)
    }
}

impl Default for PGNTags {
    fn default() -> Self {
        PGNTags {
            event: String::new(),
            site: String::new(),
            date: ChessDate::Unknown,
            round: ChessRound::default(),
            white: String::new(),
            black: String::new(),
            result: String::new(),
        }
    }
}

#[derive(Debug)]
pub enum PGNMoveTag {
    None,        // ''
    Good,        // '!'
    Excellent,   // '!!'
    Bad,         // '?'
    Blunder,     // '??'
    Interesting, // '!?'
    Doubtful,    // '?!'
}

#[derive(Debug)]
// Check = +
// Checkmate = #
pub enum CheckType {
    Check,
    CheckMate,
}

#[derive(Debug)]
// (File) OR (Rank) OR (Square)
pub struct PGNMoveSpecifier {
    rank: Option<Rank>,
    file: Option<File>,
    square: Option<SQ>,
}

#[derive(Debug)]
// [Piece](specifier)("capture")["dest"]("Promo")
// [Piece] => K, Q, R, B, N, '' if pawn
// (specifier) => rank or file or square if needed
// (capture) => x if capture
//
pub struct PGNRegMove {
    pub piece: Option<PieceType>,
    pub specifier: Option<PGNMoveSpecifier>,
    pub dest: SQ,
    pub promo: Option<PieceType>,
    pub capture: bool,
}

#[derive(Debug)]
pub enum PGNMoveType {
    KingSideCastle,  // O-O
    QueenSideCastle, // O-O-O
    Reg(PGNRegMove),
}

// (move)(check ?)(tag)
#[derive(Debug)]
pub struct PGNMove {
    pub move_type: PGNMoveType,
    pub check: Option<CheckType>,
    pub tag: PGNMoveTag,
}

impl PGNMove {
    pub fn parse(_input: &str) -> Result<PGNMove, PGNError> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct PGNRound {
    move_num: u32,
    white_move: PGNMove,
    black_move: Option<PGNMove>,
}

#[derive(Debug)]
pub enum PGNError {
    TagParse,
    Length,
}

pub struct PGN {
    tags: PGNTags,
    moves: Vec<PGNRound>,
}

// [Event "F/S Return Match"]
impl PGN {
    pub fn parse(input: &str) -> Result<PGN, PGNError> {
        let mut tags = PGNTags::default();

        let mut lines = input.lines();
        loop {
            let mut line = lines.next().ok_or(PGNError::Length)?;
            if line == "" {
                break;
            }
            line = line.trim();
            tags = tags.add(line)?;
        }
        Ok(PGN {
            tags,
            moves: Vec::new(),
        })
    }
}

#[cfg(test)]
mod tests {

    static TEST_WHITE: &'static str = "[White \"David Sr. Johnson\"]";
    static TEST_BLACK: &'static str = "[Black \"Grace Foo Bar\"]";
    static TEST_DATE: &'static str = "[Date \"2017.4.2\"]";
    static TEST_ROUND: &'static str = "[Round \"0.0\"]";
    static TEST_RESULT: &'static str = "[Round \"1-0\"]";

    extern crate rand;
    use super::*;

    #[test]
    fn tags_test() {
        PGNTags::default()
            .add(TEST_WHITE)
            .unwrap()
            .add(TEST_BLACK)
            .unwrap()
            .add(TEST_DATE)
            .unwrap()
            .add(TEST_ROUND)
            .unwrap();
    }
}
