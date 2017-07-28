/// Markerトレイトです。
pub trait Marker {
    fn new() -> Self;

    fn get_value(&self) -> i32;

    fn get_value_mut(&mut self) -> &mut i32;

    fn get_mark_at(&self, i: usize) -> i32;

    fn get_mark_mut_at(&mut self, i: usize) -> &mut i32;

    #[inline]
    fn clear(&mut self) {
        *self.get_value_mut() += 1;
    }

    #[inline]
    fn is_marked(&self, i: usize) -> bool {
        self.get_mark_at(i) == self.get_value()
    }

    #[inline]
    fn mark(&mut self, i: usize) {
        *self.get_mark_mut_at(i) = self.get_value();
    }
}

/// トレイトMarkerの具体的な構造体を宣言するマクロです。
/// $nameが構造体名, $arrayは配列サイズの定数名です。
#[macro_export]
macro_rules! make_marker {
    ($name:ident, $array:ident) => {
        pub struct $name {
            value: i32,
            marks: [i32; $array],
        }

        impl Marker for $name {
            fn new() -> Self {
                $name {
                    value: 0,
                    marks: [0; $array],
                }
            }

            #[inline]
            fn get_value(&self) -> i32 {
                self.value
            }

            #[inline]
            fn get_value_mut(&mut self) -> &mut i32 {
                &mut self.value
            }

            #[inline]
            fn get_mark_at(&self, i: usize) -> i32 {
                unsafe {
                    *self.marks.get_unchecked(i)
                }
            }

            #[inline]
            fn get_mark_mut_at(&mut self, i: usize) -> &mut i32 {
                unsafe {
                    self.marks.get_unchecked_mut(i)
                }
            }
        }
    }
}
