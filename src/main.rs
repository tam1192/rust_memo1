mod block;
use core::fmt;
use std::{
    marker,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
    u8,
};

use block::Block;

#[derive(Clone, Copy, Debug)]
enum SimpleColor {
    Red,
    Green,
    Blue,
    Yellow,
    Black,
    White,
    Purple,
}

#[derive(Clone, Copy, Debug)]
struct Color(u8, u8, u8);

impl Color {
    fn new(red: u8, green: u8, blue: u8) -> Self {
        Self(red, green, blue)
    }
    fn from_simple_color(value: &SimpleColor) -> Self {
        match value {
            SimpleColor::Red => Self(u8::MAX, 0, 0),
            SimpleColor::Green => Self(0, u8::MAX, 0),
            SimpleColor::Blue => Self(0, 0, u8::MAX),
            SimpleColor::Yellow => Self(u8::MAX, u8::MAX, 0),
            SimpleColor::Black => Self(0, 0, 0),
            SimpleColor::White => Self(u8::MAX, u8::MAX, u8::MAX),
            SimpleColor::Purple => Self(u8::MAX, 0, u8::MAX),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 > 128 {
            if self.1 > 128 {
                if self.2 > 128 {
                    // 255,255,255
                    write!(f, "⬜")
                } else {
                    // 255, 255, 0
                    write!(f, "🟨")
                }
            } else {
                if self.2 > 128 {
                    // 255, 0, 255
                    write!(f, "🟪")
                } else {
                    // 255, 0, 0
                    write!(f, "🟥")
                }
            }
        } else {
            if self.1 > 128 {
                if self.2 > 128 {
                    // 0, 255, 255
                    write!(f, "🟦")
                } else {
                    // 0, 255, 0
                    write!(f, "🟩")
                }
            } else {
                if self.2 > 128 {
                    // 0, 0, 255
                    write!(f, "🟦")
                } else {
                    // 0, 0, 0
                    write!(f, "⬛")
                }
            }
        }
    }
}

fn main() {
    // let sc = ScreenBuffer::new(10, 10, SimpleColor::Blue);
    let mut sc = Block::new(10, 10, &Color::from_simple_color(&SimpleColor::Black));
    let _ = sc.fill_box(2, 2, 8, 8, &Color::from_simple_color(&SimpleColor::Blue));
    let _ = sc.fill_box(4, 4, 6, 6, &Color::from_simple_color(&SimpleColor::Red));
    println!("{}", sc);
    sc.fill(&Color::from_simple_color(&SimpleColor::White));
    println!("{}", sc);
}
