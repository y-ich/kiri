use std::ops::Range;
use std::fmt;
use ::{Color, PointState, UsizeVec, Move, BoardError};

/// Boardの具体的な構造体のfmt::Displayのための関数です。
/// Boardを実装するtype Tでfmt::Displayを以下のように実装してください。
/// ```
/// impl fmt::Display for T {
///    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
///        board_fmt(self, f)
///    }
/// }
/// ```
pub fn board_fmt<T: Board>(p: &T, f: &mut fmt::Formatter) -> fmt::Result {
    use std::fmt::Display;

    static COL_STR: [char; 20] =
        ['@','A','B','C','D','E','F','G','H','J','K','L','M','N','O','P','Q','R','S','T'];
    let width_plus_1 = p.get_width() + 1; // inclusive rangeがないので計算しておく
    let mut result = Ok(());
    for row in 1..p.get_height() + 1 {
        result = result.and(write!(f, " {:<2} ", row));
        for col in 1..width_plus_1 { // col
            result = result.and(p.get_state(p.xy_to_linear(col as u8, row as u8)).fmt(f));
            result = result.and(write!(f, " "));
        }
        result = result.and(write!(f, "\n"));
    }
    result = result.and(write!(f, "    "));
    for col in 1..width_plus_1 {
        result = result.and(write!(f, "{} ", COL_STR[col as usize]));
    }
    result.and(write!(f, "\n\n"))
}

/// 着手の2次元座標表現です。
enum XyMove {
    /// パスです。
    Pass,
    /// 着手の2次元座標です。左上(1,1)が原点です。
    Point(u8, u8),
}

/// 代数(A1)形式の座標をusizeに変換します。
///
/// "A1"形式は左下が原点です。
///
/// ```text
/// 3 . . . . . . . . . . . . . . . . . .
/// 2 . . . . . . . . . . . . . . . . . .
/// 1 . . . . . . . . . . . . . . . . . .
///   A B C D E F G H J K L M N O P Q R S (Iは欠番)
/// ```
fn parse_algebraic(s: &str, height: usize) -> Result<XyMove, BoardError> {
    let st = s.to_uppercase();

    if st == "PASS" {
        Ok(XyMove::Pass)
    } else {
        let mut chars = st.chars();
        match chars.next() {
            Some(c) => {
                chars
                .as_str()
                .parse::<u8>()
                .map(|y| {
                    let x = c as u8 - '@' as u8 - if c < 'J' { 0 } else { 1 };
                    XyMove::Point(x, height as u8 - y + 1)
                })
                .or(Err(BoardError::InvalidVertex))
            },
            None => Err(BoardError::InvalidVertex),
        }
    }
}

/// 碁盤上の状態を操作するトレイトです。
/// ゲームのルールは加味せず、基本的なもののみです。
pub trait Board : fmt::Display {
    /// ボードの幅を返します。
    fn get_width(&self) -> usize;

    /// ボードの高さを返します。
    fn get_height(&self) -> usize;

    /// OB(Out of Bounds)領域の幅を返します。
    fn get_ob_size(&self) -> usize;

    /// ptの状態を返します。
    fn get_state(&self, pt: usize) -> PointState;

    /// ptの状態を設定します。
    fn set_state(&mut self, pt: usize, value: PointState);

    /// 次の手番を返します。
    fn get_turn(&self) -> Color;

    /// 次の手番を設定します。
    fn set_turn(&mut self, value: Color);

    /// OB含めたボードの幅を返します。
    #[inline]
    fn get_width_with_ob(&self) -> usize {
        self.get_width() + self.get_ob_size() * 2
    }

    /// xy座標をusizeに変換します。
    /// xy座標は(1,1)が原点で、左上が原点です。
    fn xy_to_linear(&self, x: u8, y: u8) -> usize {
        let x = x as usize;
        let y = y as usize;
        x + self.get_ob_size() - 1 + (y + self.get_ob_size() - 1) * self.get_width_with_ob()
    }

    /// usizeをxy座標に変換します。
    /// xy座標は(1,1)が原点で、左上が原点です。
    fn linear_to_xy(&self, p: usize) -> (u8, u8) {
        ((p % self.get_width_with_ob() - self.get_ob_size() + 1) as u8,
         (p / self.get_width_with_ob() - self.get_ob_size() + 1) as u8)
    }

    /// 次の手番を切り替えます。
    #[inline]
    fn switch_turn(&mut self) {
        let opponent = self.get_turn().opponent();
        self.set_turn(opponent);
    }

    /// 盤上の線形座標のRangeを返します。
    /// 盤外の線形座標も含みます。
    #[inline]
    fn all_points(&self) -> Range<usize> {
        self.xy_to_linear(1, 1)..self.xy_to_linear(self.get_width() as u8, self.get_height() as u8) + 1
    }

    /// 空点の配列を返します。
    #[inline]
    fn empties(&self) -> UsizeVec {
        self.all_points().filter(|&pt| self.get_state(pt) == PointState::Empty).collect()
    }

    /// 盤上かチェックします。
    #[inline]
    fn is_on_board(&self, pt: usize) -> bool {
        self.get_state(pt) != PointState::Out
    }

    /// Move(線形座標)を代数表現に変換します。
    fn str_coord(&self, mov: Move) -> String {
        match mov {
            Move::Pass     => "pass".to_string(),
            Move::Resign   => "resign".to_string(),
            Move::Linear(i) => {
                let (x, y) = self.linear_to_xy(i);
                let c = '@' as u8 + x;
                let c = if c > 'H' as u8 { c + 1 } else { c } as char;
                format!("{}{}", c, self.get_height() as u8 - y + 1)
            }
        }
    }

    /// 代数形式の座標文字列をMove(線形座標)に変換します。
    fn algebraic_to_move(&self, s: &str) -> Result<Move, BoardError> {
        parse_algebraic(s, self.get_height()).map(|xymove| match xymove {
            XyMove::Pass => Move::Pass,
            XyMove::Point(x, y) => Move::Linear(self.xy_to_linear(x, y)),
        })
    }
}
