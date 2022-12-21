#![warn(clippy::pedantic, clippy::nursery, missing_docs)]
#![allow(
    clippy::non_ascii_literal,
    clippy::use_self,
    clippy::upper_case_acronyms
)]
//! 座標、9x9の盤面（`Board`）、そしてそれに手駒を加えたもの （`Field`）などをナイーブに表す


/// Defines things in terms of relative view: "which piece is opponent's?"／相対座標ベース。「どの駒が相手の駒？」という話をする
pub mod relative;

/// Defines things in the absolute term: "which piece lies in the square LIA?"／絶対座標ベース。「LIAのマスにはどの駒がある？」という話をする
pub mod absolute;

/// Defines a perspective, with which you can transform between the absolute and the relative／視点を定めることで、相対座標と絶対座標の間を変換できるようにする
pub mod perspective;
