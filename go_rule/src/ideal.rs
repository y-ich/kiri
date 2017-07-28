pub struct Position<T: FixedSizeArray<PointState>> {
    /// コミ
    komi: f32,
    /// 盤上の状態を保持する配列
    states: T,
    /// 次の手番
    turn: Color,
    /// コウによる着手禁止点
    ko: Point,
}
