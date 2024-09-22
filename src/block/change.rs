use super::Block;

impl<T> Block<T>
where
    T: Clone,
{
    /// Sets the dot of this [`Screen<T>`].
    pub fn set_dot(&mut self, row: usize, col: usize, c: &T) {
        self.screen[self.col * row + col] = c.clone();
    }
    pub fn fill(&mut self, c: &T) {
        self.screen.fill(c.clone());
    }
    /// fill the box of this [`Screen<T>`].
    pub fn fill_box(
        &mut self,
        val_start_row: usize,
        val_start_col: usize,
        val_end_row: usize,
        val_end_col: usize,
        c: &T,
    ) {
        // 大きいほうをendに
        let (start_col, end_col) = {
            if val_start_col <= val_end_col {
                (val_start_col, val_end_col)
            } else {
                (val_end_col, val_start_col)
            }
        };
        let (start_row, end_row) = {
            if val_start_row <= val_end_row {
                (val_start_row, val_end_row)
            } else {
                (val_end_row, val_start_row)
            }
        };
        // 切り替え
        for i in start_row..end_row {
            let col = i * self.col;
            self.screen[col + start_col..col + end_col].fill(c.clone());
        }
    }
    /// write the line of this [`Screen<T>`].
    pub fn write_line(
        &mut self,
        val_start_row: usize,
        val_start_col: usize,
        val_end_row: usize,
        val_end_col: usize,
        c: &T,
    ) {
        // 参考
        // https://qiita.com/shinsa82/items/fc890aff12ed2a200e52

        // startよりendのほうが大きい場合、描画後に反転させるため、
        // flip変数に反転の有無を入れる。
        // また、endとstartを入れ替える
        let (start_col, end_col, flip_col) = {
            if val_start_col <= val_end_col {
                (val_start_col, val_end_col, false)
            } else {
                // 先に入れ替えとく
                self.col_flip();
                (val_end_col, val_start_col, true)
            }
        };
        let (start_row, end_row, flip_row) = {
            if val_start_row <= val_end_row {
                (val_start_row, val_end_row, false)
            } else {
                // 先に入れ替えとく
                self.row_flip();
                (val_end_row, val_start_row, true)
            }
        };
        // 傾き
        let delta_col = end_col - start_col;
        let delta_row = end_row - start_row;
        let mut error = 0;
        let mut row = start_row;
        for col in start_col..end_col + 1 {
            self.set_dot(row, col, &c);
            error += 2 * delta_row;
            if error > delta_col {
                row += 1;
                error -= 2 * delta_row
            }
        }
        // 反転させていた場合、元に戻す
        if flip_col {
            self.col_flip();
        }
        if flip_row {
            self.row_flip();
        }
    }
}
