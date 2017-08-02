use arrayvec::ArrayVec;
use go_board::*;

/// 着手のundoのための情報を保持する構造体です。
pub struct MoveLog {
    turn: Color,
    ko: Option<usize>,
    mov: Move,
    captives: UsizeVec,
}

/// つながった石「連(String)」を表す構造体です。
// TODO - インスタンス1つで4 * 384 * 3 = 4.6kB消費するのでなんとかしたほうがいいかもしれない。
pub struct GoString {
    pub points: UsizeVec,
    pub liberties: UsizeVec,
    pub opponents: UsizeVec,
}

impl GoString {
    pub fn new() -> GoString {
        GoString {
            points: ArrayVec::new(),
            liberties: ArrayVec::new(),
            opponents: ArrayVec::new(),
        }
    }

    fn size(&self) -> usize {
        self.points.len()
    }

    pub fn num_liberties(&self) -> usize {
        self.liberties.len()
    }

    pub fn is_nakade_shape(&self) -> bool {
        unimplemented!()
    }
}

/// 囲碁のルール沿った操作を提供するトレイトです。
/// 主に着手を扱います。
pub trait Rule : Board {
    /// コミを取得します。
    fn get_komi(&self) -> f32;

    /// コミを設定します。
    fn set_komi(&mut self, value: f32);

    /// 線形座標ptの点がコウによる着手禁止点か調べます。
    fn is_ko(&self, pt: usize) -> bool;

    /// コウによる着手禁止点を返します。
    fn get_ko(&self) -> Option<usize>;

    /// コウによる着手禁止点を設定します。
    fn set_ko(&mut self, pt: Option<usize>);

    /// 盤上が正常な局面かチェックします。
    fn check_legal(&self) -> bool {
        for pt in self.all_points() {
            if self.get_state(pt).is_stone() && self.string_at(pt).num_liberties() == 0 {
                return false;
            }
        }
        true
    }

    /// 線形座標ptの点に隣接する点の配列を返します。
    /// 盤上か盤外かは未チェックです。
    #[inline]
    fn adjacencies_at(&self, pt: usize) -> [usize; 4] {
        debug_assert!(self.is_on_board(pt), "pt = {}", pt);
        let width_plus_2 = self.get_width() + 2;
        // North East South  West
        [pt - width_plus_2, pt + 1, pt + width_plus_2, pt - 1]
    }

    /// 線形座標ptの点を含む連を返します。
    ///
    /// Markerインスタンスを使った実装を想定しているので、デフォルト実装がありません。
    /// 実装はposition.rsを参照してください。
    fn string_at(&self, pt: usize) -> GoString;

    /// 着手します。
    ///
    /// 成功するとMoveLogを返します。失敗するとエラーメッセージを返します。
    fn play(&mut self, mov: Move) -> Result<MoveLog, &'static str> {
        let ko = self.get_ko();
        match mov {
            Move::Pass => {
                self.switch_turn();
                Ok(MoveLog {
                    turn: self.get_turn(),
                    mov: Move::Pass,
                    ko: ko,
                    captives: UsizeVec::new(),
                })
            },
            Move::Linear(pt) => {
                debug_assert!(self.is_on_board(pt), "pt = {}", pt);
                if self.is_ko(pt) {
                    return Err("prohibit move");
                }

                let turn = self.get_turn();
                // 石を置き、
                self.set_state(pt, PointState::Occupied(turn));
                // ハマを上げ、
                let captives = self.capture_by(pt);
                // 自分のダメヅマリを調べる
                let string = self.string_at(pt);
                let liberties = string.num_liberties();
                if liberties == 0 { // 着手禁止点なら
                    // 着手を戻す
                    self.set_state(pt, PointState::Empty);
                    return Err("suicide move");
                }
                // コウヌキだったかチェック
                self.set_ko(if captives.len() == 1 && liberties == 1 && string.size() == 1 {
                    Some(string.liberties[0])
                } else {
                    None
                });

                self.switch_turn();
                Ok(MoveLog {
                    turn: self.get_turn(),
                    mov: Move::Linear(pt),
                    ko: ko,
                    captives: captives,
                })
            }
            _ => Err("resign is not treated by play"),
        }
    }

    /// 線形座標ptの石によって取れる石を取り上げ、その座標の配列を返します。
    fn capture_by(&mut self, pt: usize) -> UsizeVec {
        debug_assert!(self.is_on_board(pt), "out of bounds: pt = {}", pt);
        debug_assert!(self.get_state(pt).is_stone(), "should placed stone: pt = {}", pt);

        let opponent = self.get_turn().opponent();
        let mut captives = UsizeVec::new();

        for &a in &self.adjacencies_at(pt) {
            if self.get_state(a) == PointState::Occupied(opponent) {
                let string = self.string_at(a);
                if string.num_liberties() == 0 {
                    self.remove_string(&string);
                    // TODO - 配列の結合だけどそれ用のメソッドがなく要素1つ1つ追加している。もっと速い方法あり？
                    for &e in &string.points {
                        captives.push(e);
                    }
                }
            }
        }
        captives
    }

    /// 連の石を盤上から取り上げます。
    fn remove_string(&mut self, string: &GoString) {
        for &e in &string.points {
            self.set_state(e, PointState::Empty);
        }
    }

    /// 直前の着手を取り消します。
    fn undo_play(&mut self, move_log: &MoveLog) {
        self.set_ko(move_log.ko);
        self.switch_turn();
        match move_log.mov {
            Move::Linear(i) => {
                self.set_state(i, PointState::Empty);
                let opponent = move_log.turn.opponent();
                for &pt in &move_log.captives {
                    self.set_state(pt, PointState::Occupied(opponent));
                }
            }
            _ => {},
        }
    }

    /// ptに斜め隣接する位置の線形座標の配列を返します。
    #[inline]
    fn diagonal_neighbors(&self, pt: usize) -> [usize; 4] {
        debug_assert!(self.is_on_board(pt), "pt = {}", pt);
        let width_plus_2 = self.get_width() + 2;
        //  NE  SE  SW  NW
        [pt - width_plus_2 + 1, pt + width_plus_2 + 1, pt + width_plus_2 - 1, pt - width_plus_2 - 1]
    }

    /// 眼形か否かを返します。
    /// 眼になり得ればtrue、それ以外(欠け目含む)はfalseを返します。
    fn is_eye(&self, pt: usize) -> PointState {
        debug_assert!(self.is_on_board(pt), "pt = {}, {:?}", pt, self.linear_to_xy(pt));
        let mut eyecolor = PointState::Empty;
        let mut other = PointState::Empty;
        for c in self.adjacencies_at(pt).iter().map(|&n| self.get_state(n)) {
            match c {
                PointState::Out => continue,
                PointState::Empty => return PointState::Empty,
                _ => if eyecolor == PointState::Empty {
                    eyecolor = c;
                    other = c.opponent();
                } else if c == other {
                    return PointState::Empty;
                }
            }
        }

        let mut n_out = 0;
        let mut n_opponent = 0;
        for c in self.diagonal_neighbors(pt).iter().map(|&n| self.get_state(n)) {
            if c == PointState::Out {
                n_out += 1;
            } else if c == other {
                n_opponent += 1;
            }
        }
        if (n_out >= 1 &&  n_opponent >= 1) || (n_out == 0 && n_opponent >= 2) {
            PointState::Empty
        } else {
            eyecolor
        }
    }

    /// 終局を仮定して局面のスコアを返します。
    /// 盤上の石の数と眼の空点の数の差がスコアの定義です。
    ///
    /// 終局の条件は、以下の2点です。
    ///
    /// ダメ詰めが完了している。
    ///
    /// 死に石をダメを詰めて取りきっている(中国ルールを仮定)。
    fn score(&self) -> f32 {
        let mut s: i32 = 0;

        for pt in self.all_points() {
            let mut c = self.get_state(pt);
            if c == PointState::Empty {
                c = self.is_eye(pt);
            }
            if c == PointState::Occupied(Color::Black) {
                s += 1;
            } else if c == PointState::Occupied(Color::White) {
                s -= 1;
            }
        }
        s as f32 - self.get_komi()
    }
}
