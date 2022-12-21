use crate::{absolute, relative};
use cetkaik_fundamental::{AbsoluteSide, ColorAndProf};
use serde::{Deserialize, Serialize};
/// Defines a perspective, with which you can transform between the absolute and the relative
/// ／どちらの視点で見ているかを表現する型。
/// 視点を固定すると、相対座標表現と絶対座標表現を相互変換することができる。
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum Perspective {
    /// IA is the lowermost row;
    /// the player who had occupied the IA row in the beginning of the game has pieces that point upward
    /// (i.e. you)
    /// ／IAは一番下の行であり、初期状態でIA行を占有していたプレイヤーは駒が上向き（=あなた）である。
    IaIsDownAndPointsUpward,

    /// IA is the uppermost row;
    /// the player who had occupied the IA row in the beginning of the game has pieces that point downward
    /// (i.e. the opponent)
    /// ／IAは一番上の行であり、初期状態でIA行を占有していたプレイヤーは駒が下向き（=相手）である。
    IaIsUpAndPointsDownward,
}

impl Perspective {
    /// Check if IA is the lowermost row
    /// ／IAが一番下の行であるかどうかを判定する
    #[must_use]
    pub const fn ia_is_down(self) -> bool {
        matches!(self, Perspective::IaIsDownAndPointsUpward)
    }
}

/// Converts `relative::Board` into `absolute::Board`.
/// ／`relative::Board` を `absolute::Board` に変換する。
#[must_use]
pub fn to_absolute_board(board: &relative::Board, p: Perspective) -> absolute::Board {
    let mut ans = std::collections::HashMap::new();
    for (i, row) in board.0.iter().enumerate() {
        for (j, sq) in row.iter().enumerate() {
            if let Some(piece) = *sq {
                ans.insert(to_absolute_coord([i, j], p), to_absolute_piece(piece, p));
            }
        }
    }
    absolute::Board(ans)
}

/// Converts `absolute::Board` into `relative::Board`.
/// ／`absolute::Board` を `relative::Board` に変換する。
#[must_use]
pub fn to_relative_board(board: &absolute::Board, p: Perspective) -> relative::Board {
    let mut ans = [
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
    ];

    for (i, row) in ans.iter_mut().enumerate() {
        for (j, sq) in row.iter_mut().enumerate() {
            if let Some(piece) = board.0.get(&to_absolute_coord([i, j], p)) {
                *sq = Some(to_relative_piece(*piece, p));
            }
        }
    }
    relative::Board(ans)
}

/// Converts `relative::Field` into `absolute::Field`.
/// ／`relative::Field` を `absolute::Field` に変換する。
#[must_use]
pub fn to_absolute_field(field: relative::Field, p: Perspective) -> absolute::Field {
    let relative::Field {
        hop1zuo1of_downward,
        hop1zuo1of_upward,
        current_board,
    } = field;
    absolute::Field {
        board: to_absolute_board(&current_board, p),
        ia_side_hop1zuo1: match p {
            Perspective::IaIsDownAndPointsUpward => hop1zuo1of_upward
                .iter()
                .copied()
                .map(|relative::NonTam2PieceUpward { color, prof }| ColorAndProf { color, prof })
                .collect(),
            Perspective::IaIsUpAndPointsDownward => hop1zuo1of_downward
                .iter()
                .copied()
                .map(|relative::NonTam2PieceDownward { color, prof }| ColorAndProf { color, prof })
                .collect(),
        },
        a_side_hop1zuo1: match p {
            Perspective::IaIsDownAndPointsUpward => hop1zuo1of_downward
                .iter()
                .copied()
                .map(|relative::NonTam2PieceDownward { color, prof }| ColorAndProf { color, prof })
                .collect(),
            Perspective::IaIsUpAndPointsDownward => hop1zuo1of_upward
                .iter()
                .copied()
                .map(|relative::NonTam2PieceUpward { color, prof }| ColorAndProf { color, prof })
                .collect(),
        },
    }
}

/// Converts `absolute::Field` into `relative::Field`.
/// ／`absolute::Field` を `relative::Field` に変換する。
#[must_use]
pub fn to_relative_field(field: absolute::Field, p: Perspective) -> relative::Field {
    let absolute::Field {
        board,
        ia_side_hop1zuo1,
        a_side_hop1zuo1,
    } = field;

    relative::Field {
        hop1zuo1of_downward: match p {
            Perspective::IaIsUpAndPointsDownward => ia_side_hop1zuo1.iter().copied(),
            Perspective::IaIsDownAndPointsUpward => a_side_hop1zuo1.iter().copied(),
        }
        .map(|ColorAndProf { color, prof }| relative::NonTam2PieceDownward { color, prof })
        .collect(),
        hop1zuo1of_upward: match p {
            Perspective::IaIsUpAndPointsDownward => a_side_hop1zuo1.iter().copied(),
            Perspective::IaIsDownAndPointsUpward => ia_side_hop1zuo1.iter().copied(),
        }
        .map(|ColorAndProf { color, prof }| relative::NonTam2PieceUpward { color, prof })
        .collect(),
        current_board: to_relative_board(&board, p),
    }
}

/// Converts `relative::Side` into `AbsoluteSide`.
/// ／`relative::Side` を `AbsoluteSide` に変換する。
#[must_use]
pub const fn to_absolute_side(side: relative::Side, p: Perspective) -> AbsoluteSide {
    match (side, p) {
        (relative::Side::Upward, Perspective::IaIsDownAndPointsUpward)
        | (relative::Side::Downward, Perspective::IaIsUpAndPointsDownward) => AbsoluteSide::IASide,
        (relative::Side::Downward, Perspective::IaIsDownAndPointsUpward)
        | (relative::Side::Upward, Perspective::IaIsUpAndPointsDownward) => AbsoluteSide::ASide,
    }
}

/// Converts `AbsoluteSide` into `relative::Side`.
/// ／`AbsoluteSide` を `relative::Side` に変換する。
#[must_use]
pub const fn to_relative_side(side: AbsoluteSide, p: Perspective) -> relative::Side {
    match (side, p) {
        (AbsoluteSide::IASide, Perspective::IaIsDownAndPointsUpward)
        | (AbsoluteSide::ASide, Perspective::IaIsUpAndPointsDownward) => relative::Side::Upward,
        (AbsoluteSide::IASide, Perspective::IaIsUpAndPointsDownward)
        | (AbsoluteSide::ASide, Perspective::IaIsDownAndPointsUpward) => relative::Side::Downward,
    }
}

/// Converts `absolute::Piece` into `relative::Piece`.
/// ／`absolute::Piece` を `relative::Piece` に変換する。
/// # Examples
/// ```
/// use cetkaik_fundamental::*;
/// use cetkaik_naive_representation::*;
/// use cetkaik_naive_representation::perspective::*;
/// assert_eq!(
///     to_relative_piece(absolute::Piece::Tam2, Perspective::IaIsDownAndPointsUpward),
///     relative::Piece::Tam2
/// );
/// assert_eq!(
///     to_relative_piece(absolute::Piece::NonTam2Piece {
///         prof: Profession::Uai1,
///         color: Color::Kok1,
///         side: AbsoluteSide::IASide
///     }, Perspective::IaIsDownAndPointsUpward),
///     relative::Piece::NonTam2Piece {
///         prof: Profession::Uai1,
///         color: Color::Kok1,
///         side: relative::Side::Upward
///     }
/// );
/// ```
#[must_use]
pub const fn to_relative_piece(piece: absolute::Piece, p: Perspective) -> relative::Piece {
    match piece {
        absolute::Piece::Tam2 => relative::Piece::Tam2,
        absolute::Piece::NonTam2Piece { prof, color, side } => relative::Piece::NonTam2Piece {
            prof,
            color,
            side: to_relative_side(side, p),
        },
    }
}

/// Converts `relative::Piece` into `absolute::Piece`
/// ／`relative::Piece` を `absolute::Piece` に変換する。
/// # Examples
/// ```
/// use cetkaik_fundamental::*;
/// use cetkaik_naive_representation::*;
/// use cetkaik_naive_representation::perspective::*;
/// assert_eq!(
///     to_absolute_piece(relative::Piece::Tam2, Perspective::IaIsDownAndPointsUpward),
///     absolute::Piece::Tam2
/// );
/// assert_eq!(
///     to_absolute_piece(relative::Piece::NonTam2Piece {
///         prof: Profession::Uai1,
///         color: Color::Kok1,
///         side: relative::Side::Upward
///     }, Perspective::IaIsDownAndPointsUpward),
///     absolute::Piece::NonTam2Piece {
///         prof: Profession::Uai1,
///         color: Color::Kok1,
///         side: AbsoluteSide::IASide
///     }
/// );
/// ```
#[must_use]
pub const fn to_absolute_piece(piece: relative::Piece, p: Perspective) -> absolute::Piece {
    match piece {
        relative::Piece::Tam2 => absolute::Piece::Tam2,
        relative::Piece::NonTam2Piece { prof, color, side } => absolute::Piece::NonTam2Piece {
            prof,
            color,
            side: to_absolute_side(side, p),
        },
    }
}

/// Converts `relative::Coord` into `absolute::Coord`
/// ／`relative::Coord` を `absolute::Coord` に変換する。
/// # Examples
/// ```
/// use cetkaik_naive_representation::*;
/// use cetkaik_naive_representation::perspective::*;
/// assert_eq!(
///     to_absolute_coord([2, 4], Perspective::IaIsDownAndPointsUpward),
///     absolute::Coord(absolute::Row::I, absolute::Column::Z)
/// )
/// ```
#[must_use]
pub fn to_absolute_coord(coord: relative::Coord, p: Perspective) -> absolute::Coord {
    let [row, col] = coord;

    let columns = vec![
        absolute::Column::K,
        absolute::Column::L,
        absolute::Column::N,
        absolute::Column::T,
        absolute::Column::Z,
        absolute::Column::X,
        absolute::Column::C,
        absolute::Column::M,
        absolute::Column::P,
    ];

    let rows = vec![
        absolute::Row::A,
        absolute::Row::E,
        absolute::Row::I,
        absolute::Row::U,
        absolute::Row::O,
        absolute::Row::Y,
        absolute::Row::AI,
        absolute::Row::AU,
        absolute::Row::IA,
    ];

    super::absolute::Coord(
        rows[if p.ia_is_down() { row } else { 8 - row }],
        columns[if p.ia_is_down() { col } else { 8 - col }],
    )
}

/// Converts `absolute::Coord` into `relative::Coord`
/// ／`absolute::Coord` を `relative::Coord` に変換する。
/// # Examples
/// ```
/// use cetkaik_naive_representation::*;
/// use cetkaik_naive_representation::perspective::*;
/// assert_eq!(
///     to_relative_coord(absolute::Coord(absolute::Row::I, absolute::Column::Z), Perspective::IaIsDownAndPointsUpward),
///     [2, 4]
/// )
/// ```
#[must_use]
pub const fn to_relative_coord(coord: absolute::Coord, p: Perspective) -> relative::Coord {
    let super::absolute::Coord(row, col) = coord;

    let columns_col = match col {
        absolute::Column::K => 0,
        absolute::Column::L => 1,
        absolute::Column::N => 2,
        absolute::Column::T => 3,
        absolute::Column::Z => 4,
        absolute::Column::X => 5,
        absolute::Column::C => 6,
        absolute::Column::M => 7,
        absolute::Column::P => 8,
    };

    let rows_row = match row {
        absolute::Row::A => 0,
        absolute::Row::E => 1,
        absolute::Row::I => 2,
        absolute::Row::U => 3,
        absolute::Row::O => 4,
        absolute::Row::Y => 5,
        absolute::Row::AI => 6,
        absolute::Row::AU => 7,
        absolute::Row::IA => 8,
    };

    if p.ia_is_down() {
        [rows_row, columns_col]
    } else {
        [8 - rows_row, 8 - columns_col]
    }
}
