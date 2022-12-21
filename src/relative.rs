use cetkaik_fundamental::{Color, Profession};
use cetkaik_traits::{IsBoard, IsField};

/// Describes which player it is
/// ／どちら側のプレイヤーであるかを指定する。
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Side {
    /// The player whose pieces point upward in your perspective, i.e. yours.
    /// ／君の視点で駒が上を向いている駒、つまり、君の駒。
    Upward,

    /// The player whose pieces point downward in your perspective, i.e. the opponent's.
    /// ／君の視点で駒が下を向いている駒、つまり、相手の駒。
    Downward,
}

impl std::ops::Not for Side {
    type Output = Side;

    fn not(self) -> Self::Output {
        match self {
            Side::Upward => Side::Downward,
            Side::Downward => Side::Upward,
        }
    }
}

/// Describes a piece that is not a Tam2 and points downward (i.e. opponents).
/// ／駒のうち、皇ではなくて、下向き（つまり相手陣営）のものを表す。
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct NonTam2PieceDownward {
    /// color of the piece／駒の色
    pub color: Color,
    /// profession of the piece／駒の職種
    pub prof: Profession,
}

/// Describes a piece that is not a Tam2 and points upward (i.e. yours).
/// ／駒のうち、皇ではなくて、上向き（つまり自分陣営）のものを表す。
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct NonTam2PieceUpward {
    /// color of the piece／駒の色
    pub color: Color,
    /// profession of the piece／駒の職種
    pub prof: Profession,
}

impl From<NonTam2PieceUpward> for Piece {
    fn from(from: NonTam2PieceUpward) -> Piece {
        Piece::NonTam2Piece {
            color: from.color,
            prof: from.prof,
            side: Side::Upward,
        }
    }
}

impl From<NonTam2PieceDownward> for Piece {
    fn from(from: NonTam2PieceDownward) -> Piece {
        Piece::NonTam2Piece {
            color: from.color,
            prof: from.prof,
            side: Side::Downward,
        }
    }
}

/// Describes a piece on the board.
/// ／盤上に存在できる駒を表現する。
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Piece {
    /// Tam2, a special piece belonging to both sides. Both players can move it.
    /// ／皇（たむ）。自分も相手も動かすことができる共有の駒である。
    Tam2,

    /// All the other usual pieces that belong to a single side.
    /// ／残りの全ての普通の駒。片方の陣営にのみ属する。
    NonTam2Piece {
        /// color of the piece／駒の色
        color: Color,
        /// profession of the piece／駒の職種
        prof: Profession,

        /// which side the piece belongs to
        /// ／駒の所属側。どちらの陣営に属しているのかを表す。
        side: Side,
    },
}

impl Piece {
    /// Checks whether the piece is a Tam2.
    /// ／皇であるかどうかの判定
    #[must_use]
    pub const fn is_tam2(self) -> bool {
        match self {
            Piece::Tam2 => true,
            Piece::NonTam2Piece { .. } => false,
        }
    }

    /// Checks whether the piece has a specific color. Tam2 has neither color.
    /// ／駒が特定の色であるかを調べる。皇は赤でも黒でもない。
    #[must_use]
    pub fn has_color(self, clr: Color) -> bool {
        match self {
            Piece::Tam2 => false,
            Piece::NonTam2Piece { color, .. } => color == clr,
        }
    }

    /// Checks whether the piece has a specific profession.
    /// ／駒が特定の職種であるかを調べる。
    #[must_use]
    pub fn has_prof(self, prf: Profession) -> bool {
        match self {
            Piece::Tam2 => false,
            Piece::NonTam2Piece { prof, .. } => prof == prf,
        }
    }

    /// Checks whether the piece belongs to a specific side. Tam2 belongs to neither side.
    /// ／駒が特定の側のプレイヤーに属するかどうかを調べる。皇はどちらの陣営にも属さない。
    #[must_use]
    pub fn has_side(self, sid: Side) -> bool {
        match self {
            Piece::Tam2 => false,
            Piece::NonTam2Piece { side, .. } => side == sid,
        }
    }
}

#[must_use]
fn rotate_piece_or_null(p: Option<Piece>) -> Option<Piece> {
    let p = p?;
    match p {
        Piece::Tam2 => Some(p),
        Piece::NonTam2Piece { prof, color, side } => Some(Piece::NonTam2Piece {
            prof,
            color,
            side: !side,
        }),
    }
}

/// Denotes the position of a square by [row, col].
/// ／マス目の相対座標を [row, col] で表す。
///
pub type Coord = [usize; 2];

/// Serializes [`Coord`](./type.Coord.html) in JSON-style.
/// ／[`Coord`](./type.Coord.html) を JSON スタイルで文字列にする。
/// # Examples
/// ```
/// use cetkaik_naive_representation::relative::*;
///
/// assert_eq!(serialize_coord([5,6]), "[5,6]")
/// ```
#[must_use]
pub fn serialize_coord(coord: Coord) -> String {
    format!("[{},{}]", coord[0], coord[1])
}

/// Rotates the coordinate with the center of the board as the center of rotation.
/// ／盤の中心を基準に、座標を180度回転させる。
#[must_use]
pub const fn rotate_coord(c: Coord) -> Coord {
    [(8 - c[0]), (8 - c[1])]
}

/// Checks if the square is a tam2 nua2 (tam2's water), entry to which is restricted.
/// ／マスが皇水（たむぬあ）であるかどうかの判定
#[must_use]
#[allow(clippy::nonminimal_bool)]
pub const fn is_water([row, col]: Coord) -> bool {
    (row == 4 && col == 2)
        || (row == 4 && col == 3)
        || (row == 4 && col == 4)
        || (row == 4 && col == 5)
        || (row == 4 && col == 6)
        || (row == 2 && col == 4)
        || (row == 3 && col == 4)
        || (row == 5 && col == 4)
        || (row == 6 && col == 4)
}

const fn serialize_side(side: Side) -> &'static str {
    match side {
        Side::Upward => "↑",
        Side::Downward => "↓",
    }
}

/// Serializes [`Piece`](./enum.Piece.html).
/// ／[`Piece`](./enum.Piece.html) を文字列にする。
/// # Examples
/// ```
/// use cetkaik_fundamental::*;
/// use cetkaik_naive_representation::relative::*;
///
/// assert_eq!(serialize_piece(Piece::Tam2), "皇");
/// assert_eq!(serialize_piece(Piece::NonTam2Piece {
///     prof: Profession::Uai1,
///     color: Color::Kok1,
///     side: Side::Downward
/// }), "赤将↓");
/// ```
#[must_use]
pub fn serialize_piece(p: Piece) -> String {
    match p {
        Piece::Tam2 => "皇".to_string(),
        Piece::NonTam2Piece { prof, color, side } => format!(
            "{}{}{}",
            cetkaik_fundamental::serialize_color(color),
            cetkaik_fundamental::serialize_prof(prof),
            serialize_side(side)
        ),
    }
}

/// Describes the board, the 9x9 squares, in terms of relative coordinates.
/// ／盤、つまり、9x9のマス目を、相対座標で表す。
#[derive(Clone, Debug, Eq, PartialEq, Hash, Copy)]
pub struct Board(pub [SingleRow; 9]);

/// Describes a single row made up of 9 squares.
/// ／横一列の9マス、を表す。
pub type SingleRow = [Option<Piece>; 9];

/// Describes the field, which is defined as a board plus each side's hop1zuo1.
/// ／フィールドを表す。フィールドとは、盤に両者の手駒を加えたものである。
#[derive(Debug, Clone, Hash)]
pub struct Field {
    /// board／盤
    pub current_board: Board,

    /// hop1zuo1 for the Upward (i.e. you)／Upward側（あなた）の手駒
    pub hop1zuo1of_upward: Vec<NonTam2PieceUpward>,

    /// hop1zuo1 for the Downward (i.e. opponent)／Downward側（相手）の手駒
    pub hop1zuo1of_downward: Vec<NonTam2PieceDownward>,
}

/// Returns the initial configuration as specified in the y1 huap1 (the standardized rule).
/// The red king points upward (i.e. you)
/// ／官定で定められた初期配置を与える。赤王が自分側にある。
#[must_use]
pub fn yhuap_initial_board_where_red_king_points_upward() -> Board {
    rotate_board(&yhuap_initial_board_where_black_king_points_upward())
}

/// Returns the initial configuration as specified in the y1 huap1 (the standardized rule).
/// The black king points upward (i.e. you)
/// ／官定で定められた初期配置を与える。黒王が自分側にある。
#[must_use]
#[allow(clippy::too_many_lines)]
pub const fn yhuap_initial_board_where_black_king_points_upward() -> Board {
    Board([
        [
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Kua2,
                side: Side::Downward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Maun1,
                side: Side::Downward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Kaun1,
                side: Side::Downward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Uai1,
                side: Side::Downward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Io,
                side: Side::Downward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Uai1,
                side: Side::Downward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Kaun1,
                side: Side::Downward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Maun1,
                side: Side::Downward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Kua2,
                side: Side::Downward,
            }),
        ],
        [
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Tuk2,
                side: Side::Downward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Gua2,
                side: Side::Downward,
            }),
            None,
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Dau2,
                side: Side::Downward,
            }),
            None,
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Dau2,
                side: Side::Downward,
            }),
            None,
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Gua2,
                side: Side::Downward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Tuk2,
                side: Side::Downward,
            }),
        ],
        [
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Kauk2,
                side: Side::Downward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Kauk2,
                side: Side::Downward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Kauk2,
                side: Side::Downward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Kauk2,
                side: Side::Downward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Nuak1,
                side: Side::Downward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Kauk2,
                side: Side::Downward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Kauk2,
                side: Side::Downward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Kauk2,
                side: Side::Downward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Kauk2,
                side: Side::Downward,
            }),
        ],
        [None, None, None, None, None, None, None, None, None],
        [
            None,
            None,
            None,
            None,
            Some(Piece::Tam2),
            None,
            None,
            None,
            None,
        ],
        [None, None, None, None, None, None, None, None, None],
        [
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Kauk2,
                side: Side::Upward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Kauk2,
                side: Side::Upward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Kauk2,
                side: Side::Upward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Kauk2,
                side: Side::Upward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Nuak1,
                side: Side::Upward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Kauk2,
                side: Side::Upward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Kauk2,
                side: Side::Upward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Kauk2,
                side: Side::Upward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Kauk2,
                side: Side::Upward,
            }),
        ],
        [
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Tuk2,
                side: Side::Upward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Gua2,
                side: Side::Upward,
            }),
            None,
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Dau2,
                side: Side::Upward,
            }),
            None,
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Dau2,
                side: Side::Upward,
            }),
            None,
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Gua2,
                side: Side::Upward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Tuk2,
                side: Side::Upward,
            }),
        ],
        [
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Kua2,
                side: Side::Upward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Maun1,
                side: Side::Upward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Kaun1,
                side: Side::Upward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Uai1,
                side: Side::Upward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Io,
                side: Side::Upward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Uai1,
                side: Side::Upward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Kaun1,
                side: Side::Upward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Maun1,
                side: Side::Upward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Kua2,
                side: Side::Upward,
            }),
        ],
    ])
}

impl Field {
    /// Add a piece to one's hop1zuo1.
    /// ／手駒に駒を追加する。
    pub fn insert_nontam_piece_into_hop1zuo1(
        &mut self,
        color: Color,
        prof: Profession,
        side: Side,
    ) {
        match side {
            Side::Upward => self
                .hop1zuo1of_upward
                .push(NonTam2PieceUpward { color, prof }),
            Side::Downward => self
                .hop1zuo1of_downward
                .push(NonTam2PieceDownward { color, prof }),
        }
    }
}

/// Rotates a board.
/// ／盤を180度回転させ、自分陣営と相手陣営を入れ替える。
#[must_use]
pub fn rotate_board(b: &Board) -> Board {
    let mut ans: Board = Board([
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
    ]);
    for i in 0..9 {
        for j in 0..9 {
            ans.0[i][j] = rotate_piece_or_null(b.0[8 - i][8 - j]);
        }
    }
    ans
}

/// Calculates the distance between two points.
/// The distance is defined as the larger of the difference between either the x or y coordinates.
/// ／2点間の距離（x座標の差およびy座標の差のうち小さくない方）を計算する。
/// # Examples
/// ```
/// use cetkaik_naive_representation::relative::*;
/// assert_eq!(5, distance([4,5], [4,0]));
/// assert_eq!(3, distance([4,5], [1,2]));
/// assert_eq!(3, distance([1,2], [4,5]));
/// ```
///
/// # Panics
/// Panics if the `Coord` is so invalid that it does not fit in `i32`.
/// ／`Coord` に入っている座標が `i32` に収まらないほど巨大であれば panic する。
#[must_use]
pub fn distance(a: Coord, b: Coord) -> i32 {
    let [x1, y1] = a;
    let [x2, y2] = b;

    let x_distance = (i32::try_from(x1).unwrap() - i32::try_from(x2).unwrap()).abs();
    let y_distance = (i32::try_from(y1).unwrap() - i32::try_from(y2).unwrap()).abs();

    x_distance.max(y_distance)
}

/// Describes a move denoted in absolute coordinates.
/// ／絶対座標で書かれた指し手を表す。
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PureMove {
    /// A non-Tam2 piece moves from a square on a board to another square without stepping.
    /// ／皇ではない駒が、盤上から盤上に踏越えなしで移動する。
    NonTamMoveSrcDst {
        /// origin／開始点
        src: Coord,
        /// final destination／終了点
        dest: Coord,
        /// whether a water-entry ciurl is required／入水判定が必要かどうか
        is_water_entry_ciurl: bool,
    },
    /// A non-Tam2 piece moves from a square on a board to another square, during which it steps another piece and does a finite movement.
    /// ／皇ではない駒が、盤上から盤上に踏越えを伴いながら移動し、踏越え後は有限移動をする。
    NonTamMoveSrcStepDstFinite {
        /// origin／開始点
        src: Coord,
        /// via point／経由点
        step: Coord,
        /// destination／終了点
        dest: Coord,
        /// whether a water-entry ciurl is required／入水判定が必要かどうか
        is_water_entry_ciurl: bool,
    },
    /// A non-Tam2 piece moves from a square on a board to another square, during which it steps another piece and tries to do a directional, infinite movement.
    /// ／皇ではない駒が、盤上から盤上に踏越えを伴いながら移動し、踏越え後は無限移動をしようとする。
    InfAfterStep {
        /// origin／開始点
        src: Coord,
        /// via point／経由点
        step: Coord,
        /// the planned LOCATION. After casting the sticks, some rules necessitates that you go to the destination or to the direction that you have declared beforehand.
        /// Hence the confusing name.
        /// ／行く予定の場所。踏越え判定後の移動先は、ルールによっては「計画したマス」である必要があったり「計画した方角」である必要があったりする。
        planned_direction: Coord,
    },
    /// A non-Tam2 piece moves from hop1zuo1 to a square on a board.
    /// ／皇ではない駒が、手駒から盤上に移動する。
    NonTamMoveFromHopZuo {
        /// color／駒の色
        color: Color,
        /// profession／駒の役職
        prof: Profession,
        /// destination／終了点
        dest: Coord,
    },
    /// A Tam2 moves from a square on a board to another square without stepping.
    /// ／皇が盤上から盤上に踏越えなしで移動する。
    TamMoveNoStep {
        /// origin／開始点
        src: Coord,
        /// first destination／一回目の終了点
        first_dest: Coord,
        /// second destination／二回目の終了点
        second_dest: Coord,
    },
    /// A Tam2 moves from a square on a board to another square. In the former half of its movement, it steps another piece.
    /// ／皇が盤上から盤上に移動し、一回目の移動の最中に踏越えをする。
    TamMoveStepsDuringFormer {
        /// origin／開始点
        src: Coord,
        /// via point／経由点
        step: Coord,
        /// first destination／一回目の終了点
        first_dest: Coord,
        /// second destination／二回目の終了点
        second_dest: Coord,
    },
    /// A Tam2 moves from a square on a board to another square. In the latter half of its movement, it steps another piece.
    /// ／皇が盤上から盤上に移動し、二回目の移動の最中に踏越えをする。
    TamMoveStepsDuringLatter {
        /// origin／開始点
        src: Coord,
        /// via point／経由点
        step: Coord,
        /// first destination／一回目の終了点
        first_dest: Coord,
        /// second destination／二回目の終了点
        second_dest: Coord,
    },
}

impl PureMove {
    /// Serializes [`PureMove`](./enum.PureMove.html) in textual form.
    /// # Examples
    /// ```
    /// use cetkaik_fundamental::*;
    /// use cetkaik_naive_representation::*;
    /// use cetkaik_naive_representation::relative::*;
    ///
    /// assert_eq!(PureMove::InfAfterStep {
    ///     src: [0, 4],
    ///     step: [1, 3],
    ///     planned_direction: [1, 2]
    /// }.serialize(), "[0,4]片[1,3]心[1,2]");
    ///
    /// assert_eq!(PureMove::NonTamMoveFromHopZuo {
    ///     color: Color::Huok2,
    ///     prof: Profession::Gua2,
    ///     dest: [8, 1]
    /// }.serialize(), "黒弓[8,1]");
    ///
    /// assert_eq!(PureMove::NonTamMoveSrcDst {
    ///     src: [0, 4],
    ///     dest: [1, 2],
    ///     is_water_entry_ciurl: true
    /// }.serialize(), "[0,4]片[1,2]水");
    ///
    /// assert_eq!(PureMove::NonTamMoveSrcStepDstFinite {
    ///     src: [0, 4],
    ///     step: [1, 3],
    ///     dest: [1, 2],
    ///     is_water_entry_ciurl: false
    /// }.serialize(), "[0,4]片[1,3][1,2]");
    ///
    /// // Note that [] denotes the first destination.
    /// // Since the first destination is neither the stepping square nor the final square,
    /// // it is not to be written in the standard notation.
    /// // Hence this additional information is denoted by [].
    /// assert_eq!(PureMove::TamMoveStepsDuringFormer {
    ///     src: [1, 0],
    ///     step: [2, 1],
    ///     first_dest: [2, 0],
    ///     second_dest: [1, 1]
    /// }.serialize(), "[1,0]皇[2,1][[2,0]][1,1]");
    ///
    /// assert_eq!(PureMove::TamMoveNoStep {
    ///     src: [1, 0],
    ///     first_dest: [2, 0],
    ///     second_dest: [1, 0]
    /// }.serialize(), "[1,0]皇[[2,0]][1,0]");
    ///
    /// assert_eq!(PureMove::TamMoveStepsDuringLatter {
    ///     src: [1, 0],
    ///     first_dest: [2, 0],
    ///     step: [2, 1],
    ///     second_dest: [1, 1]
    /// }.serialize(), "[1,0]皇[[2,0]][2,1][1,1]");
    /// ```
    #[must_use]
    pub fn serialize(self) -> String {
        match self {
            PureMove::InfAfterStep {
                src,
                step,
                planned_direction,
            } => format!(
                "{}片{}心{}",
                serialize_coord(src),
                serialize_coord(step),
                serialize_coord(planned_direction)
            ),
            PureMove::NonTamMoveFromHopZuo { color, prof, dest } => format!(
                "{}{}{}",
                cetkaik_fundamental::serialize_color(color),
                cetkaik_fundamental::serialize_prof(prof),
                serialize_coord(dest)
            ),
            PureMove::NonTamMoveSrcDst {
                src,
                dest,
                is_water_entry_ciurl,
            } => format!(
                "{}片{}{}",
                serialize_coord(src),
                serialize_coord(dest),
                if is_water_entry_ciurl { "水" } else { "" }
            ),
            PureMove::NonTamMoveSrcStepDstFinite {
                src,
                dest,
                is_water_entry_ciurl,
                step,
            } => format!(
                "{}片{}{}{}",
                serialize_coord(src),
                serialize_coord(step),
                serialize_coord(dest),
                if is_water_entry_ciurl { "水" } else { "" }
            ),
            PureMove::TamMoveNoStep {
                src,
                first_dest,
                second_dest,
            } => format!(
                "{}皇[{}]{}",
                serialize_coord(src),
                serialize_coord(first_dest),
                serialize_coord(second_dest)
            ),
            PureMove::TamMoveStepsDuringFormer {
                src,
                first_dest,
                second_dest,
                step,
            } => format!(
                "{}皇{}[{}]{}",
                serialize_coord(src),
                serialize_coord(step),
                serialize_coord(first_dest),
                serialize_coord(second_dest)
            ),
            PureMove::TamMoveStepsDuringLatter {
                src,
                first_dest,
                second_dest,
                step,
            } => format!(
                "{}皇[{}]{}{}",
                serialize_coord(src),
                serialize_coord(first_dest),
                serialize_coord(step),
                serialize_coord(second_dest)
            ),
        }
    }
}

impl IsBoard for Board {
    type PieceWithSide = Piece;

    type Coord = Coord;

    fn peek(&self, c: Self::Coord) -> Option<Self::PieceWithSide> {
        self.0[c[0]][c[1]]
    }

    fn pop(&mut self, c: Self::Coord) -> Option<Self::PieceWithSide> {
        let o = self.peek(c);
        self.0[c[0]][c[1]] = None;
        o
    }

    fn put(&mut self, c: Self::Coord, p: Option<Self::PieceWithSide>) {
        self.0[c[0]][c[1]] = p;
    }

    fn assert_empty(&self, c: Self::Coord) {
        assert!(self.peek(c).is_none());
    }

    fn assert_occupied(&self, c: Self::Coord) {
        assert!(self.peek(c).is_some());
    }
}

impl IsField for Field {
    type Board = Board;
    type Coord = Coord;
    type PieceWithSide = Piece;
    type Side = Side;

    fn move_nontam_piece_from_src_to_dest_while_taking_opponent_piece_if_needed(
        &self,
        src: Self::Coord,
        dest: Self::Coord,
        whose_turn: Self::Side,
    ) -> Result<Self, &'static str>
    where
        Self: std::marker::Sized,
    {
        let mut new_self = self.clone();
        let src_piece =
            new_self.current_board.0[src[0]][src[1]].ok_or("src does not contain a piece")?;

        let Piece::NonTam2Piece { color: _color, prof: _prof, side } = src_piece 
        else {
            return Err("Expected a NonTam2Piece to be present at the src, but found a Tam2")
        };

        if whose_turn != side {
            return Err("Found the opponent piece at the src");
        }

        let maybe_captured_piece = new_self.current_board.0[dest[0]][dest[1]];
        new_self.current_board.0[dest[0]][dest[1]] = Some(src_piece);

        if let Some(captured_piece) = maybe_captured_piece {
            match captured_piece {
                Piece::Tam2 => return Err("Tried to capture a Tam2"),
                Piece::NonTam2Piece {
                    color: captured_piece_color,
                    prof: captured_piece_prof,
                    side: captured_piece_side,
                } => {
                    if captured_piece_side == whose_turn {
                        return Err("Tried to capture an ally");
                    }
                    match whose_turn {
                        Side::Downward => new_self.hop1zuo1of_downward.push(NonTam2PieceDownward {
                            color: captured_piece_color,
                            prof: captured_piece_prof,
                        }),
                        Side::Upward => new_self.hop1zuo1of_upward.push(NonTam2PieceUpward {
                            color: captured_piece_color,
                            prof: captured_piece_prof,
                        }),
                    }
                }
            }
        }
        Ok(new_self)
    }

    fn as_board(&self) -> &Self::Board {
        &self.current_board
    }

    fn as_board_mut(&mut self) -> &mut Self::Board {
        &mut self.current_board
    }

    #[must_use]
    fn search_from_hop1zuo1_and_parachute_at(
        &self,
        color: Color,
        prof: Profession,
        side: Side,
        to: Coord,
    ) -> Option<Self> {
        match side {
            Side::Upward => {
                let mut new_self = self.clone();
                let index = new_self
                    .hop1zuo1of_upward
                    .iter()
                    .position(|x| *x == NonTam2PieceUpward { color, prof })?;
                new_self.hop1zuo1of_upward.remove(index);

                if self.current_board.0[to[0]][to[1]].is_some() {
                    return None;
                }
                new_self.current_board.0[to[0]][to[1]] =
                    Some(Piece::NonTam2Piece { color, prof, side });
                Some(new_self)
            }
            Side::Downward => {
                let mut new_self = self.clone();
                let index = new_self
                    .hop1zuo1of_downward
                    .iter()
                    .position(|x| *x == NonTam2PieceDownward { color, prof })?;
                new_self.hop1zuo1of_downward.remove(index);

                if self.current_board.0[to[0]][to[1]].is_some() {
                    return None;
                }
                new_self.current_board.0[to[0]][to[1]] =
                    Some(Piece::NonTam2Piece { color, prof, side });
                Some(new_self)
            }
        }
    }
}
