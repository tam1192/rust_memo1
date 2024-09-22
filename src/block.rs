use std::{
    fmt,
    ops::{Deref, DerefMut},
};
mod change;
mod impls;

// スクリーンの実装
#[derive(Clone, Debug)]
pub struct Block<T> {
    // 二次元配列を一次元で表現
    screen: Vec<T>,
    // 行(y)
    row: usize,
    // 列(x)
    col: usize,
}

impl<T> Block<T>
where
    T: Clone,
{
    /// Creates a new [`Screen<T>`].
    pub fn new(row: usize, col: usize, default: &T) -> Self {
        // 二次元を一次元で表現する
        let screen: Vec<_> = (0..row * col).map(|_| default.clone()).collect();
        Self { screen, row, col }
    }
}
impl<T> Block<T> {
    /// Returns the get row of this [`Screen<T>`].
    pub fn get_row(&self) -> usize {
        self.row
    }
    /// Returns the get col of this [`Screen<T>`].
    pub fn get_col(&self) -> usize {
        self.col
    }
    /// Returns the col flip of this [`Screen<T>`].
    // 左右入れ替え
    pub fn col_flip(&mut self) {
        // 行の数が1以上
        if self.col > 1 {
            // 左側用プローブ
            let mut probe1 = 0;
            // 右側用プローブ
            let mut probe2 = self.col - 1;
            loop {
                // 1行ごと入れ替える
                for row in 0..self.row {
                    // 一行当たりの項目数がself.col
                    // n行飛ばすときはその分を掛けて飛ばす
                    let r = self.col * row;
                    // swapはローコストで入れ替えてくれる
                    // (copy, cloneが不要)
                    self.screen.swap(probe1 + r, probe2 + r);
                }
                // プローブ（列）をずらす
                // 中心に寄せていく
                probe1 += 1;
                probe2 -= 1;
                // プローブが重なったor中心を超えた場合は
                if probe1 >= probe2 {
                    // 完了
                    break;
                }
            }
        }
    }
    /// Returns the row flip of this [`Screen<T>`].
    pub fn row_flip(&mut self) {
        // 行が1以上
        if self.row > 1 {
            // 上側用プローブ
            let mut probe1 = 0;
            // 下側用プローブ
            let mut probe2 = self.row - 1;
            loop {
                // 一列ごと入れ替える
                for col in 0..self.col {
                    // swapを使うとローコスト
                    // 一行あたりの項目数がself.colに当たる。
                    // 例えば、列数が3だった時
                    // 0行目 = 0 * 3 = 0
                    // 1行目 = 1 * 3 = 3
                    // 2行目 = 2 * 3 = 6
                    // さらにcolを足せば、列をずらすこともできる。
                    // 0行目 = 0 * 3 + 1 = 1
                    // 1行目 = 1 * 3 + 1 = 4
                    // ...
                    self.screen
                        .swap(probe1 * self.col + col, probe2 * self.col + col);
                }
                // プローブ（列）をずらす
                // 中心に寄せていく
                probe1 += 1;
                probe2 -= 1;
                // プローブが重なったor中心を超えた場合は
                if probe1 >= probe2 {
                    // 完了
                    break;
                }
            }
        }
    }
}