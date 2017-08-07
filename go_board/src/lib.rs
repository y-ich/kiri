//! 盤上の基本操作をトレイトとして提供します。
//! 基本操作なので、ルールは実装されていません。(囲碁にも五目並べにも使えます軽量モジュールです)
//!
//! 速度を重視して、盤上の状態を保持する1次元配列を線形座標でアクセスする方式を採りました。
//!
//! 1次元配列は、下図のように盤上(.)盤外(#, OB)領域を持つ2次元構造を1次元に繋いだものです。
//!
//! (9路盤,盤外の幅=2の場合の例)
//!
//! ```text
//! #############
//! #############
//! ##.........##
//! ##.........##
//! ##.........##
//! ##.........##
//! ##.........##
//! ##.........##
//! ##.........##
//! ##.........##
//! ##.........##
//! #############
//! #############
//! ```
//!
//! # 使い方
//! ```
//! use std::fmt;
//! use go_board::*;
//!
//! const BOARD_SIZE: usize = 15;
//! const OB_SIZE: usize = 1;
//! const ARRAY_SIZE: usize = array_size!(BOARD_SIZE, OB_SIZE);
//! struct MyGoban {
//!     turn: Color,
//!     state: [PointState; ARRAY_SIZE],
//! }
//! impl Board for MyGoban {
//!     fn get_width(&self) -> usize { BOARD_SIZE }
//!     fn get_height(&self) -> usize { BOARD_SIZE }
//!     fn get_ob_size(&self) -> usize { OB_SIZE }
//!     fn get_state(&self, pt: usize) -> PointState { self.state[pt] }
//!     fn set_state(&mut self, pt: usize, value: PointState) { self.state[pt] = value; }
//!     fn get_turn(&self) -> Color { self.turn }
//!     fn set_turn(&mut self, value: Color) { self.turn = value }
//! }
//! impl fmt::Display for MyGoban {
//!    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//!        board_fmt(self, f)
//!    }
//! }
//! ```

extern crate arrayvec;

use std::fmt;
use arrayvec::ArrayVec;

mod error;
pub use error::*;
mod board;
pub use board::*;
mod marker;
pub use marker::*;

/// 盤サイズの配列確保用の定数を生成するマクロです。
/// $sizeは碁盤のサイズ、$ob_sizeはOBの幅です。
#[macro_export]
macro_rules! array_size {
    ($size:expr, $ob_size:expr) => {
        ($size + $ob_size * 2) * ($size + $ob_size * 2);
    }
}

/// 着手を表す列挙型です。
//  TODO - enum使わずusizeで盤外の値をPass, Resignに割り当てたほうが速い。
//         enumの読みやすさで、usizeに最適化される書き方を探す。
pub enum Move {
    /// パスです。
    Pass,
    /// 投了です。
    Resign,
    /// 着手した交点の線形座標です。
    Linear(usize),
}

/// 盤上の交点の可変長集合を扱うタイプです。
pub type UsizeVec = ArrayVec<[usize; 384]>;

/// 手番や石の色の列挙型です。
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn opponent(&self) -> Self {
        match *self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Color::Black => "Black",
            Color::White => "White",
        })
    }
}

/// 盤上の各交点の状態の列挙型です。
/// 交点はIntersectionですが、長いのでPointとしています。
// TODO - きっとビットパターンで指定したほうが速くなる処理がある。読みやすさと速さの両立。
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum PointState {
    /// 空点です。
    Empty,
    /// 石が存在しています。
    Occupied(Color),
    /// 盤外です。
    Out,
    /// 着手禁止点です。
    Forbidden,
}

impl PointState {
    /// 種類を反転(黒なら白、白なら黒)させた値を返します。
    // TODO - Resultを使うべきか否か。
    pub fn opponent(&self) -> Self {
        match *self {
            PointState::Occupied(c) => PointState::Occupied(c.opponent()),
            _                       => PointState::Forbidden,
        }
    }

    /// 石か否かを返します。
    #[inline]
    pub fn is_stone(&self) -> bool {
        if let PointState::Occupied(_) = *self {
            true
        } else {
            false
        }
    }

    /// 文字に対応するPointStateを返します。
    pub fn from_char(c: char) -> PointState {
        match c {
            'X' => PointState::Occupied(Color::Black),
            'O' => PointState::Occupied(Color::White),
            '#' => PointState::Out,
             _  => PointState::Empty,
        }
    }
}

impl fmt::Display for PointState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            PointState::Empty | PointState::Forbidden => '.',
            PointState::Occupied(Color::Black)        => 'X',
            PointState::Occupied(Color::White)        => 'O',
            PointState::Out                           => '#',
        })
    }
}

#[cfg(test)]
mod tests {
    use ::*;

    #[test]
    fn test_point_state_size() {
        use std::mem;
        // TODO - 今の実装ではサイズは2。1に最適化できるようなコードを探す。
        assert_eq!(1, mem::size_of::<PointState>());
    }
}
