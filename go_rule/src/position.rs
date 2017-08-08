use std::fmt;
use std::mem::uninitialized;
use go_board::*;
use rule::*;

/// 盤上の状況を表す構造体PositionXX(XXは盤サイズ)を宣言するマクロです。
/// $nameが構造体名、$sizeは碁盤のサイズ, $arrayは配列サイズの定数名です。
macro_rules! make_position {
    ($size:expr, $ob_size:expr, $name:ident, $array:ident, $marker:ident, $marker_instance:ident) => {
        const $array: usize = array_size!($size, $ob_size);
        make_marker!($marker, $array);

        /// 共有Markerインスタンスです。
        pub static mut $marker_instance: $marker = $marker {
            value: 0,
            marks: [0; $array],
        };

        /// 盤上の局面を表す構造体です。
        #[allow(dead_code)]
        #[derive(Copy)]
        pub struct $name {
            /// コミ
            komi: f32,
            /// 盤上の状態を保持する配列
            states: [PointState; $array],
            /// 次の手番
            turn: Color,
            /// コウによる着手禁止点
            ko: Option<LinearCoord>,
        }

        impl Clone for $name {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                board_fmt(self, f)
            }
        }

        impl Board for $name {
            #[inline]
            fn get_width(&self) -> LinearCoord {
                $size
            }

            #[inline]
            fn get_height(&self) -> LinearCoord {
                $size
            }

            #[inline]
            fn get_ob_size(&self) -> LinearCoord {
                $ob_size
            }

            #[inline]
            fn get_state(&self, pt: LinearCoord) -> PointState {
                unsafe {
                    *self.states.get_unchecked(pt as usize)
                }
            }

            #[inline]
            fn set_state(&mut self, pt: LinearCoord, value: PointState) {
                unsafe {
                    let elem = self.states.get_unchecked_mut(pt as usize);
                    *elem = value;
                }
            }

            #[inline]
            fn get_turn(&self) -> Color {
                self.turn
            }

            #[inline]
            fn set_turn(&mut self, value: Color) {
                self.turn = value;
            }
        }

        impl Rule for $name {
            #[inline]
            fn get_komi(&self) -> f32 {
                self.komi
            }

            #[inline]
            fn set_komi(&mut self, value: f32) {
                self.komi = value;
            }

            #[inline]
            fn is_ko(&self, pt: LinearCoord) -> bool {
                match self.ko {
                    Some(i) => pt == i,
                    None    => false,
                }
            }

            #[inline]
            fn get_ko(&self) -> Option<LinearCoord> {
                self.ko
            }

            #[inline]
            fn set_ko(&mut self, pt: Option<LinearCoord>) {
                self.ko = pt;
            }

            fn string_at(&self, pt: LinearCoord, string: &mut GoString) {
                debug_assert!(self.is_on_board(pt), "pt = {}", pt);
                let stone = self.get_state(pt);
                debug_assert!(stone.is_stone(), "no stones");

                unsafe {
                    $marker_instance.clear();

                    string.points.push(pt);
                    let mut index = 0;
                    while index < string.points.len() {
                        let pt = string.points[index];
                        let upt = pt as usize;
                        if !$marker_instance.is_marked(upt) {
                            $marker_instance.mark(upt);
                            for &a in &self.adjacencies_at(pt) {
                                let ua = a as usize;
                                if !$marker_instance.is_marked(ua) {
                                    let state = self.get_state(a);
                                    if state == stone {
                                        string.points.push(a);
                                    } else {
                                        $marker_instance.mark(ua);
                                        if state == PointState::Empty {
                                            string.liberties.push(a);
                                        }
                                    }
                                }
                            }
                        }
                        index += 1;
                    }
                }
            }
        }

        impl $name {
             pub fn new() -> Self {
                let mut pos: Self = unsafe { uninitialized() };
                pos.reset();
                pos
            }

            /// 内部状態をデフォルト値に設定します。
            fn reset(&mut self) {
                for pt in 0..self.states.len() {
                    self.set_state(pt as LinearCoord, PointState::Out);
                }
                for row in 1..self.get_height() + 1 {
                    for col in 1..self.get_width() + 1 {
                        let pt = self.xy_to_linear(col as u8, row as u8);
                        self.set_state(pt, PointState::Empty);
                    }
                }
                self.set_ko(None);
                self.set_turn(Color::Black);
                self.set_komi(6.5);
            }

            /// 盤上の文字表現から$nameのインスタンスを返します。
            /// 以下は盤上の文字表現は4路盤の例です。
            ///
            /// ```text
            /// ....
            /// ..X.
            /// .O..
            /// ....
            /// ```
            pub fn from_string(s: &str) -> Result<Self, &str> {
                let mut pos = Self::new();
                if s.lines().count() != pos.get_height() as usize {
                    return Err("wrong rows");
                }
                let lines = s.lines();
                let width = pos.get_width();
                for (y, line) in lines.enumerate() {
                    if line.len() != width as usize {
                        return Err("wrong columns");
                    }
                    for (x, c) in line.chars().enumerate() {
                        let i = pos.xy_to_linear(x as u8 + 1, y as u8 + 1);
                        pos.set_state(i, PointState::from_char(c));
                    }
                }
                Ok(pos)
            }
        }
    }
}

// マクロを使って19路盤のstructを定義します。
// Rust(1.20.0)では識別子を合成して定義に使うことができないので、必要な識別子を引数に与えています。
make_position!(19, 1, Position19, ARRAY_SIZE_19, Marker19, MARKER19);
