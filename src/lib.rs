use std::{fmt, error};


#[derive(Debug,PartialEq)]
pub enum GameError {
    Select(SelectError),
    Move(MoveError)
}

#[derive(Debug,PartialEq)]
pub enum SelectError {
    WrongColor,
    OutsideBoard,
    NoPieceAtPosition,
}
impl From<SelectError> for GameError {
    fn from(error: SelectError) -> Self {
        GameError::Select(error)
    }
}

#[derive(Debug,PartialEq)]
pub enum MoveError {
    SquareOcupied,
    InvalidMove,
}
impl From<MoveError> for GameError {
    fn from(error: MoveError) -> Self {
        GameError::Move(error)
    }
}

#[derive(Copy,Clone)]
pub struct Position {
    row: usize,
    col: usize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Result<Self,SelectError> {
        if (row >= 8)  || (col >= 8) {
            return Err(SelectError::OutsideBoard);
        }
        Ok(Position { row, col })
    }
}

pub struct Board {
    pub tiles: [[Tile;8];8],
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        for row in self.tiles {
            for tile in row {
                write!(f," ")?;
                if let Some(piece) = tile.piece {
                    match piece.piece_type {
                       PieceType::Rook => write!(f,"r")?,
                       PieceType::Bishop => write!(f,"b")?,
                       PieceType::Pawn=> write!(f,"p")?,
                       PieceType::King => write!(f,"K")?,
                       PieceType::Knight=>  write!(f,"k")?,
                       PieceType::Queen => write!(f,"Q")?,
                    };
                }
                if let None = tile.piece {
                    write!(f,".")?;
                }
            }
            println!("");
        }
        return Ok(())
    }
}

impl Board {
    pub fn new() -> Self{
        Board {
            tiles: [[Tile::default(); 8]; 8],
        }
    }
    fn get_tile(&self, position: Position) -> Tile{
        self.tiles[position.row][position.col]
    }
    pub fn can_move(&self, from: Position, to: Position) -> Result<bool,MoveError> {
        let start_tile = self.get_tile(from);
        let end_tile = self.get_tile(to);
        match (start_tile.piece, end_tile.piece) {
            (Some(start_piece), Some(end_piece)) if start_piece.piece_color == end_piece.piece_color => Err(MoveError::SquareOcupied),
            _ => Ok(true)
        }
    }
    pub fn move_piece(&mut self, from: Position, to: Position) -> Result<(), MoveError> {
        if self.can_move(from, to)? {
            let start_tile = self.get_tile(from);
            self.tiles[from.row][from.col].insert(None);
            self.tiles[to.row][to.col].insert(start_tile.piece);
        }
        Ok(())
    }
    pub fn select_piece<T>(&self, position: Position, player: Player) -> Result<T, SelectError>
        where
        T: Piece,
    {
        match self.get_tile(position).piece {
            None => Err(SelectError::NoPieceAtPosition),
            Some(piece) if piece.piece_color != player.color => Err(SelectError::WrongColor),
            Some(piece) => Ok(piece),
        }
    }
}


pub struct Player {
    color: Color,
}

#[derive(Clone,Copy, Debug)]
pub enum PieceType{
    Pawn,
    Knight,
    Bishop,
    Queen,
    King,
    Rook,
}

#[derive(Clone,Copy, Debug,PartialEq)]
pub enum Color {
    White,
    Black,
}

trait Piece {
    fn new(piece_color: Color) -> Self;
    fn get_moves(position:Position, board: &Board) -> Vec<Position>;
    fn get_type(&self) -> PieceType;
    fn get_color(&self) -> Color {
        self.color
    }
}

pub struct Pawn{
    color:Color,
    has_moved: bool,
    has_double_moved: bool,
}
impl Pawn {
    fn can_enpessant(&self, position: Position, board: Board) -> bool {
    }
}
impl Piece for Pawn {
    fn new(&self,color: Color) -> Self {
        Pawn { color, has_moved: false, has_double_moved: false }
    }
    fn get_type(&self) -> PieceType {
        PieceType::Pawn
    }
    fn get_moves(position:Position, board: &Board) -> Vec<Position> {
        vec![Position::new(0,3)]
    }
}
pub struct Rook {
    color:Color,
    has_moved: bool,
}
impl Piece for Rook {
    fn new(color: Color) -> Self {
        Rook { color, has_moved: false }
    }
    fn get_type(&self) -> PieceType {
        PieceType::Rook
    }
    fn get_moves(position:Position, board: &Board) -> Vec<Position> {
        let mut possible_moves: Vec<Position> = Vec::new();

    }
}

pub struct Knight {
    color:Color
}
pub struct Bishop {
    color:Color
}
pub struct Queen{
    color:Color
}
pub struct King{
    color:Color,
    has_moved: bool,
}



#[derive(Clone,Copy,Debug)]
pub struct Tile {
    pub piece: Option<Piece>,
}
impl Tile {
    pub fn new(piece: Option<Piece>) -> Self {
        Tile {piece}
    }

    pub fn insert(&mut self, piece: Option<Piece>) {
        self.piece = piece;
    }
}
impl Default for Tile {
    fn default() -> Self {
        Tile::new(None)
    }
}
