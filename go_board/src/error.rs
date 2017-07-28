/// 盤上の操作で起こるエラーの種類です。
#[derive(Debug)]
pub enum BoardError {
    /// 座標変換絡みのエラーです。
    InvalidVertex,
}
