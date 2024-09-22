use std::{
    fmt,
    ops::{Deref, DerefMut},
};

use super::Block;

impl<T> Deref for Block<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.screen
    }
}

impl<T> DerefMut for Block<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.screen
    }
}

// displayの実装には、Screenの中身がdisplayを実装している必要がある。
impl<T> fmt::Display for Block<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = String::with_capacity((self.row + 1) * self.col);
        for i in 0..self.row * self.col {
            if i % self.col == 0 {
                buf.push_str("\n");
            }
            buf.push_str(&self.screen[i].to_string());
        }
        write!(f, "{}", buf)
    }
}
