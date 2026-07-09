// src/main.rs
use Color::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Color {
    Red,
    Blue,
    Yellow,
    Green,
}

impl Color {
    pub fn next(&self) -> Color {
        match self {
            Color::Red => Color::Blue,
            Color::Blue => Color::Yellow,
            Color::Yellow => Color::Green,
            Color::Green => Color::Red,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType,
}

pub type Board = [[Option<Piece>; 14]; 14];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Move {
    pub from: (usize, usize),
    pub to: (usize, usize),
    pub piece: PieceType,
    pub captured: Option<Piece>,
    pub promotion: Option<PieceType>,
}

pub struct GameState {
    pub board: Board,
    pub turn: Color,
}

impl GameState {
    pub fn new() -> Self {
        let mut board: Board = [[None; 14]; 14];

        // Красные (снизу, y=12,13)
        let red_back = [
            PieceType::Rook, PieceType::Knight, PieceType::Bishop, PieceType::Queen,
            PieceType::King, PieceType::Bishop, PieceType::Knight, PieceType::Rook
        ];
        for (i, pt) in red_back.iter().enumerate() {
            board[13][3 + i] = Some(Piece { color: Color::Red, piece_type: *pt });
            board[12][3 + i] = Some(Piece { color: Color::Red, piece_type: PieceType::Pawn });
        }

        // Жёлтые (сверху, y=0,1)
        for (i, pt) in red_back.iter().enumerate() {
            board[0][3 + i] = Some(Piece { color: Color::Yellow, piece_type: *pt });
            board[1][3 + i] = Some(Piece { color: Color::Yellow, piece_type: PieceType::Pawn });
        }

        // Синие (слева, x=0,1)
        let blue_back = [
            PieceType::Rook, PieceType::Knight, PieceType::Bishop, PieceType::King,
            PieceType::Queen, PieceType::Bishop, PieceType::Knight, PieceType::Rook
        ];
        for (i, pt) in blue_back.iter().enumerate() {
            board[3 + i][0] = Some(Piece { color: Color::Blue, piece_type: *pt });
            board[3 + i][1] = Some(Piece { color: Color::Blue, piece_type: PieceType::Pawn });
        }

        // Зелёные (справа, x=12,13)
        for (i, pt) in blue_back.iter().enumerate() {
            board[3 + i][13] = Some(Piece { color: Color::Green, piece_type: *pt });
            board[3 + i][12] = Some(Piece { color: Color::Green, piece_type: PieceType::Pawn });
        }

        Self { board, turn: Color::Red }
    }

    // Проверка, принадлежит ли клетка игровой доске (отсекаем углы 3x3)
    pub fn is_valid_square(x: i32, y: i32) -> bool {
        if x < 0 || x > 13 || y < 0 || y > 13 {
            return false;
        }
        let in_x_wings = x < 3 || x > 10;
        let in_y_wings = y < 3 || y > 10;
        !(in_x_wings && in_y_wings)
    }

    // Направление "вперёд" для пешек каждого цвета
    fn get_forward_vector(&self, color: Color) -> (i32, i32) {
        match color {
            Color::Red => (0, -1),
            Color::Yellow => (0, 1),
            Color::Blue => (1, 0),
            Color::Green => (-1, 0),
        }
    }

    // Является ли клетка (x,y) клеткой превращения для пешки цвета `color`
    fn is_promotion_square(x: i32, y: i32, color: Color) -> bool {
        match color {
            Color::Red => y == 3,
            Color::Yellow => y == 10,
            Color::Blue => x == 10,
            Color::Green => x == 3,
        }
    }

    // Проверяет, атакована ли клетка (x,y) фигурами цвета `attacker_color`
    fn is_square_attacked(&self, x: i32, y: i32, attacker_color: Color) -> bool {
        for row in 0..14 {
            for col in 0..14 {
                if let Some(piece) = self.board[row][col] {
                    if piece.color == attacker_color {
                        let from = (col, row);
                        // Для каждой фигуры проверяем, может ли она пойти на (x,y) (только взятие)
                        match piece.piece_type {
                            PieceType::Pawn => {
                                // Пешка атакует по диагоналям "вперёд"
                                let (fx, fy) = self.get_forward_vector(attacker_color);
                                let diag1 = (from.0 as i32 + fx + fy, from.1 as i32 + fy - fx);
                                let diag2 = (from.0 as i32 + fx - fy, from.1 as i32 + fy + fx);
                                if (diag1.0 == x && diag1.1 == y) || (diag2.0 == x && diag2.1 == y) {
                                    return true;
                                }
                            }
                            PieceType::Knight => {
                                let offsets = [(1,2), (1,-2), (-1,2), (-1,-2), (2,1), (2,-1), (-2,1), (-2,-1)];
                                for (dx, dy) in offsets {
                                    let nx = from.0 as i32 + dx;
                                    let ny = from.1 as i32 + dy;
                                    if nx == x && ny == y && Self::is_valid_square(nx, ny) {
                                        return true;
                                    }
                                }
                            }
                            PieceType::Bishop => {
                                for (dx, dy) in [(1,1), (1,-1), (-1,1), (-1,-1)] {
                                    let mut nx = from.0 as i32 + dx;
                                    let mut ny = from.1 as i32 + dy;
                                    while Self::is_valid_square(nx, ny) {
                                        if nx == x && ny == y {
                                            return true;
                                        }
                                        if self.board[ny as usize][nx as usize].is_some() {
                                            break;
                                        }
                                        nx += dx;
                                        ny += dy;
                                    }
                                }
                            }
                            PieceType::Rook => {
                                for (dx, dy) in [(1,0), (-1,0), (0,1), (0,-1)] {
                                    let mut nx = from.0 as i32 + dx;
                                    let mut ny = from.1 as i32 + dy;
                                    while Self::is_valid_square(nx, ny) {
                                        if nx == x && ny == y {
                                            return true;
                                        }
                                        if self.board[ny as usize][nx as usize].is_some() {
                                            break;
                                        }
                                        nx += dx;
                                        ny += dy;
                                    }
                                }
                            }
                            PieceType::Queen => {
                                for (dx, dy) in [(1,1), (1,-1), (-1,1), (-1,-1), (1,0), (-1,0), (0,1), (0,-1)] {
                                    let mut nx = from.0 as i32 + dx;
                                    let mut ny = from.1 as i32 + dy;
                                    while Self::is_valid_square(nx, ny) {
                                        if nx == x && ny == y {
                                            return true;
                                        }
                                        if self.board[ny as usize][nx as usize].is_some() {
                                            break;
                                        }
                                        nx += dx;
                                        ny += dy;
                                    }
                                }
                            }
                            PieceType::King => {
                                for (dx, dy) in [(-1,-1), (-1,0), (-1,1), (0,-1), (0,1), (1,-1), (1,0), (1,1)] {
                                    let nx = from.0 as i32 + dx;
                                    let ny = from.1 as i32 + dy;
                                    if nx == x && ny == y && Self::is_valid_square(nx, ny) {
                                        return true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        false
    }

    // Находится ли король цвета `color` под шахом
    fn is_king_in_check(&self, color: Color) -> bool {
        // Ищем короля
        for y in 0..14 {
            for x in 0..14 {
                if let Some(piece) = self.board[y][x] {
                    if piece.color == color && piece.piece_type == PieceType::King {
                        // Проверяем, атакует ли эту клетку любой другой цвет
                        for other_color in [Red, Blue, Yellow, Green] {
                            if other_color != color && self.is_square_attacked(x as i32, y as i32, other_color) {
                                return true;
                            }
                        }
                        return false;
                    }
                }
            }
        }
        false // Если короля нет (не должно случаться)
    }

    // Генерация легальных ходов (псевдолегальные с фильтрацией шаха)
    pub fn generate_legal_moves(&self) -> Vec<Move> {
        let mut moves = self.generate_pseudo_legal_moves();
        moves.retain(|mv| {
            // Применяем ход на временной доске и проверяем шах
            let mut board_copy = self.board;
            let piece = board_copy[mv.from.1][mv.from.0].unwrap();
            board_copy[mv.to.1][mv.to.0] = Some(piece);
            board_copy[mv.from.1][mv.from.0] = None;
            if let Some(prom) = mv.promotion {
                board_copy[mv.to.1][mv.to.0] = Some(Piece { color: piece.color, piece_type: prom });
            }
            // Проверяем, не находится ли король под шахом после хода
            // (используем копию состояния без полноценного GameState)
            let temp_state = GameState { board: board_copy, turn: self.turn };
            !temp_state.is_king_in_check(self.turn)
        });
        moves
    }

    // Генерация псевдолегальных ходов (без учёта шаха)
    pub fn generate_pseudo_legal_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        let color = self.turn;

        for y in 0..14 {
            for x in 0..14 {
                if let Some(piece) = self.board[y][x] {
                    if piece.color == color {
                        let from = (x, y);
                        match piece.piece_type {
                            PieceType::Pawn => self.generate_pawn_moves(from, color, &mut moves),
                            PieceType::Knight => self.generate_knight_moves(from, color, &mut moves),
                            PieceType::Bishop => self.generate_sliding_moves(from, color, &[(1,1), (1,-1), (-1,1), (-1,-1)], &mut moves),
                            PieceType::Rook => self.generate_sliding_moves(from, color, &[(1,0), (-1,0), (0,1), (0,-1)], &mut moves),
                            PieceType::Queen => self.generate_sliding_moves(from, color, &[(1,1), (1,-1), (-1,1), (-1,-1), (1,0), (-1,0), (0,1), (0,-1)], &mut moves),
                            PieceType::King => self.generate_king_moves(from, color, &mut moves),
                        }
                    }
                }
            }
        }
        moves
    }

    fn generate_pawn_moves(&self, from: (usize, usize), color: Color, moves: &mut Vec<Move>) {
        let (x, y) = (from.0 as i32, from.1 as i32);
        let (fx, fy) = self.get_forward_vector(color);

        // 1. Ход на 1 клетку вперёд
        let one_step = (x + fx, y + fy);
        if Self::is_valid_square(one_step.0, one_step.1) {
            if self.board[one_step.1 as usize][one_step.0 as usize].is_none() {
                let prom = if Self::is_promotion_square(one_step.0, one_step.1, color) {
                    Some(PieceType::Queen)
                } else {
                    None
                };
                moves.push(Move {
                    from,
                    to: (one_step.0 as usize, one_step.1 as usize),
                    piece: PieceType::Pawn,
                    captured: None,
                    promotion: prom,
                });

                // 2. Двойной ход с начальной позиции
                let (start_x, start_y) = match color {
                    Red => (0, 12),
                    Yellow => (0, 1),
                    Blue => (1, 0),
                    Green => (12, 0),
                };
                let is_start = match color {
                    Red | Yellow => y == start_y,
                    Blue | Green => x == start_x,
                };
                if is_start {
                    let two_step = (x + fx * 2, y + fy * 2);
                    if Self::is_valid_square(two_step.0, two_step.1) &&
                       self.board[two_step.1 as usize][two_step.0 as usize].is_none() {
                        moves.push(Move {
                            from,
                            to: (two_step.0 as usize, two_step.1 as usize),
                            piece: PieceType::Pawn,
                            captured: None,
                            promotion: None, // на двойном ходе нет превращения
                        });
                    }
                }
            }
        }

        // 3. Взятия по диагоналям
        let diag1 = (x + fx + fy, y + fy - fx);
        let diag2 = (x + fx - fy, y + fy + fx);

        for target in [diag1, diag2] {
            if Self::is_valid_square(target.0, target.1) {
                if let Some(target_piece) = self.board[target.1 as usize][target.0 as usize] {
                    if target_piece.color != color {
                        let prom = if Self::is_promotion_square(target.0, target.1, color) {
                            Some(PieceType::Queen)
                        } else {
                            None
                        };
                        moves.push(Move {
                            from,
                            to: (target.0 as usize, target.1 as usize),
                            piece: PieceType::Pawn,
                            captured: Some(target_piece),
                            promotion: prom,
                        });
                    }
                }
            }
        }
    }

    fn generate_knight_moves(&self, from: (usize, usize), color: Color, moves: &mut Vec<Move>) {
        let (x, y) = (from.0 as i32, from.1 as i32);
        let offsets = [(1,2), (1,-2), (-1,2), (-1,-2), (2,1), (2,-1), (-2,1), (-2,-1)];
        for (dx, dy) in offsets {
            let nx = x + dx;
            let ny = y + dy;
            if Self::is_valid_square(nx, ny) {
                let target = self.board[ny as usize][nx as usize];
                if target.is_none() || target.unwrap().color != color {
                    moves.push(Move {
                        from,
                        to: (nx as usize, ny as usize),
                        piece: PieceType::Knight,
                        captured: target,
                        promotion: None,
                    });
                }
            }
        }
    }

    fn generate_king_moves(&self, from: (usize, usize), color: Color, moves: &mut Vec<Move>) {
        let (x, y) = (from.0 as i32, from.1 as i32);
        let offsets = [(-1,-1), (-1,0), (-1,1), (0,-1), (0,1), (1,-1), (1,0), (1,1)];
        for (dx, dy) in offsets {
            let nx = x + dx;
            let ny = y + dy;
            if Self::is_valid_square(nx, ny) {
                let target = self.board[ny as usize][nx as usize];
                if target.is_none() || target.unwrap().color != color {
                    moves.push(Move {
                        from,
                        to: (nx as usize, ny as usize),
                        piece: PieceType::King,
                        captured: target,
                        promotion: None,
                    });
                }
            }
        }
    }

    fn generate_sliding_moves(&self, from: (usize, usize), color: Color, directions: &[(i32, i32)], moves: &mut Vec<Move>) {
        let (x, y) = (from.0 as i32, from.1 as i32);
        let piece_type = self.board[y as usize][x as usize].unwrap().piece_type;

        for (dx, dy) in directions {
            let mut nx = x + dx;
            let mut ny = y + dy;
            while Self::is_valid_square(nx, ny) {
                let target = self.board[ny as usize][nx as usize];
                if let Some(target_piece) = target {
                    if target_piece.color != color {
                        moves.push(Move {
                            from,
                            to: (nx as usize, ny as usize),
                            piece: piece_type,
                            captured: Some(target_piece),
                            promotion: None,
                        });
                    }
                    break;
                } else {
                    moves.push(Move {
                        from,
                        to: (nx as usize, ny as usize),
                        piece: piece_type,
                        captured: None,
                        promotion: None,
                    });
                }
                nx += dx;
                ny += dy;
            }
        }
    }
}

fn main() {
    let game = GameState::new();
    let legal_moves = game.generate_legal_moves();
    println!("Игрок: {:?}", game.turn);
    println!("Сгенерировано легальных ходов: {}", legal_moves.len());
    for m in legal_moves.iter().take(5) {
        println!("{:?}", m);
    }
}