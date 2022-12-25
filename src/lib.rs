#![warn(clippy::pedantic, clippy::nursery, missing_docs)]
#![allow(
    clippy::non_ascii_literal,
    clippy::use_self,
    clippy::upper_case_acronyms
)]
//! 座標、9x9の盤面（`Board`）、そしてそれに手駒を加えたもの （`Field`）などをナイーブに表す

use cetkaik_fundamental::{Profession, ColorAndProf, AbsoluteSide};
use cetkaik_traits::{CetkaikRepresentation, IsBoard};


/// Defines things in terms of relative view: "which piece is opponent's?"／相対座標ベース。「どの駒が相手の駒？」という話をする
pub mod relative;

/// Defines things in the absolute term: "which piece lies in the square LIA?"／絶対座標ベース。「LIAのマスにはどの駒がある？」という話をする
pub mod absolute;

/// Defines a perspective, with which you can transform between the absolute and the relative／視点を定めることで、相対座標と絶対座標の間を変換できるようにする
pub mod perspective;

/// `cetkaik_naive_representation` クレートを表すためのマーカー型
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct CetkaikNaive;

/// `cetkaik_naive_representation` クレートに基づいており、視点に依らない絶対座標での表現と、視点に依る相対座標への表現を正しく相互変換できる。
impl CetkaikRepresentation for CetkaikNaive {
    type Perspective = crate::perspective::Perspective;

    type AbsoluteCoord = crate::absolute::Coord;
    type RelativeCoord = crate::relative::Coord;

    type AbsoluteBoard = crate::absolute::Board;
    type RelativeBoard = crate::relative::Board;

    type AbsolutePiece = crate::absolute::Piece;
    type RelativePiece = crate::relative::Piece;

    type RelativeSide = crate::relative::Side;

    type AbsoluteField = crate::absolute::Field;
    type RelativeField = crate::relative::Field;

    fn to_absolute_coord(coord: Self::RelativeCoord, p: Self::Perspective) -> Self::AbsoluteCoord {
        crate::perspective::to_absolute_coord(coord, p)
    }
    fn add_delta(
        coord: Self::RelativeCoord,
        row_delta: isize,
        col_delta: isize,
    ) -> Option<Self::RelativeCoord> {
        let [i, j] = coord;
        match (
            i.checked_add_signed(row_delta),
            j.checked_add_signed(col_delta),
        ) {
            (Some(l @ 0..=8), Some(m @ 0..=8)) => Some([l, m]),
            _ => None,
        }
    }
    fn relative_get(
        board: Self::RelativeBoard,
        coord: Self::RelativeCoord,
    ) -> Option<Self::RelativePiece> {
        let [i, j] = coord;
        board.0[i][j]
    }
    fn relative_clone_and_set(
        board: &Self::RelativeBoard,
        coord: Self::RelativeCoord,
        p: Option<Self::RelativePiece>,
    ) -> Self::RelativeBoard {
        let [i, j] = coord;
        let mut new_board = *board;
        new_board.0[i][j] = p;
        new_board
    }
    fn absolute_get(
        board: &Self::AbsoluteBoard,
        coord: Self::AbsoluteCoord,
    ) -> Option<Self::AbsolutePiece> {
        board.0.get(&coord).copied()
    }
    fn is_tam_hue_by_default(coord: Self::RelativeCoord) -> bool {
        coord == [2, 2]
            || coord == [2, 6]
            || coord == [3, 3]
            || coord == [3, 5]
            || coord == [4, 4]
            || coord == [5, 3]
            || coord == [5, 5]
            || coord == [6, 2]
            || coord == [6, 6]
    }
    fn relative_tam2() -> Self::RelativePiece {
        crate::relative::Piece::Tam2
    }
    fn absolute_tam2() -> Self::AbsolutePiece {
        crate::absolute::Piece::Tam2
    }
    fn is_upward(s: Self::RelativeSide) -> bool {
        s == crate::relative::Side::Upward
    }
    fn match_on_piece_and_apply<U>(
        piece: Self::RelativePiece,
        f_tam: &dyn Fn() -> U,
        f_piece: &dyn Fn(Profession, Self::RelativeSide) -> U,
    ) -> U {
        match piece {
            Self::RelativePiece::Tam2 => f_tam(),
            Self::RelativePiece::NonTam2Piece {
                color: _,
                prof,
                side,
            } => f_piece(prof, side),
        }
    }
    fn empty_squares_relative(
        board: &crate::relative::Board,
    ) -> Vec<crate::relative::Coord> {
        let mut ans = vec![];
        for rand_i in 0..9 {
            for rand_j in 0..9 {
                let coord: crate::relative::Coord = [rand_i, rand_j];
                if board.peek(coord).is_none() {
                    ans.push(coord);
                }
            }
        }
        ans
    }
    fn empty_squares_absolute(board: &crate::absolute::Board) -> Vec<Self::AbsoluteCoord> {
        use absolute::Column::{C, K, L, M, N, P, T, X, Z};
        use absolute::Row::{A, AI, AU, E, I, IA, O, U, Y};
        let mut ans = vec![];
        for row in &[A, E, I, U, O, Y, AI, AU, IA] {
            for column in &[K, L, N, T, Z, X, C, M, P] {
                let coord = absolute::Coord(*row, *column);
                if board.peek(coord).is_none() {
                    ans.push(coord);
                }
            }
        }
        ans
    }
    fn hop1zuo1_of(
        side: cetkaik_fundamental::AbsoluteSide,
        field: &Self::AbsoluteField,
    ) -> Vec<ColorAndProf> {
        match side {
            AbsoluteSide::IASide => field.ia_side_hop1zuo1.clone(),
            AbsoluteSide::ASide => field.a_side_hop1zuo1.clone(),
        }
    }
    fn as_board_absolute(field: &Self::AbsoluteField) -> &Self::AbsoluteBoard {
        &field.board
    }
    fn as_board_mut_absolute(field: &mut Self::AbsoluteField) -> &mut Self::AbsoluteBoard {
        &mut field.board
    }
    fn as_board_relative(field: &Self::RelativeField) -> &Self::RelativeBoard {
        &field.current_board
    }
    fn is_water_relative(c: Self::RelativeCoord) -> bool {
        crate::relative::is_water(c)
    }
    fn is_water_absolute(c: Self::AbsoluteCoord) -> bool {
        crate::absolute::is_water(c)
    }
    fn loop_over_one_side_and_tam(
        board: &Self::RelativeBoard,
        side: Self::RelativeSide,
        f_tam_or_piece: &mut dyn FnMut(Self::RelativeCoord, Option<Profession>),
    ) {
        for (rand_i, row) in board.0.iter().enumerate() {
            for (rand_j, &piece) in row.iter().enumerate() {
                let src = [rand_i, rand_j];
                if let Some(p) = piece {
                    match p {
                        Self::RelativePiece::Tam2 => f_tam_or_piece(src, None),
                        Self::RelativePiece::NonTam2Piece {
                            side: side_,
                            prof,
                            color: _,
                        } if side_ == side => f_tam_or_piece(src, Some(prof)),
                        Self::RelativePiece::NonTam2Piece { .. } => {}
                    }
                }
            }
        }
    }
    fn to_relative_field(field: Self::AbsoluteField, p: Self::Perspective) -> Self::RelativeField {
        crate::perspective::to_relative_field(field, p)
    }
    fn to_relative_side(side: AbsoluteSide, p: Self::Perspective) -> Self::RelativeSide {
        crate::perspective::to_relative_side(side, p)
    }
    fn get_one_perspective() -> Self::Perspective {
        // arbitrary
        crate::perspective::Perspective::IaIsDownAndPointsUpward
    }
    fn absolute_distance(a: Self::AbsoluteCoord, b: Self::AbsoluteCoord) -> i32 {
        crate::absolute::distance(a, b)
    }
    fn absolute_same_direction(
        origin: Self::AbsoluteCoord,
        a: Self::AbsoluteCoord,
        b: Self::AbsoluteCoord,
    ) -> bool {
        crate::absolute::same_direction(origin, a, b)
    }
    fn has_prof_absolute(piece: Self::AbsolutePiece, prof: Profession) -> bool {
        piece.has_prof(prof)
    }

    fn match_on_relative_piece_and_apply<U>(
        piece: Self::RelativePiece,
        f_tam: &dyn Fn() -> U,
        f_piece: &dyn Fn(cetkaik_fundamental::Color, Profession, Self::RelativeSide) -> U,
    ) -> U {
        match piece {
            Self::RelativePiece::Tam2 => f_tam(),
            Self::RelativePiece::NonTam2Piece {
                color,
                prof,
                side,
            } => f_piece(color, prof, side),
        }
    }

    fn match_on_absolute_piece_and_apply<U>(
        piece: Self::AbsolutePiece,
        f_tam: &dyn Fn() -> U,
        f_piece: &dyn Fn(cetkaik_fundamental::Color, Profession, cetkaik_fundamental::AbsoluteSide) -> U,
    ) -> U {
        match piece {
            absolute::Piece::Tam2 => f_tam(),
            absolute::Piece::NonTam2Piece { color, prof, side } => f_piece(color, prof, side),
        }
    }
}
