use chrono;
use core::fmt;
use std::{
    marker,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

#[derive(Copy)]
enum SimpleColor {
    Red,
    Green,
    Blue,
    Yellow,
    Brown,
}
impl Clone for SimpleColor {
    fn clone(&self) -> Self {
        match self {
            SimpleColor::Blue => SimpleColor::Blue,
            SimpleColor::Brown => SimpleColor::Brown,
            SimpleColor::Green => SimpleColor::Green,
            SimpleColor::Red => SimpleColor::Red,
            SimpleColor::Yellow => SimpleColor::Yellow,
        }
    }
}
impl fmt::Display for SimpleColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SimpleColor::Blue => write!(f, "🟦"),
            SimpleColor::Brown => write!(f, "🟫"),
            SimpleColor::Green => write!(f, "🟩"),
            SimpleColor::Red => write!(f, "🟥"),
            SimpleColor::Yellow => write!(f, "🟨"),
        }
    }
}

struct ScreenBuffer<T: Copy + fmt::Display> {
    screen: Vec<Arc<Mutex<Vec<T>>>>,
}
impl<T> ScreenBuffer<T>
where
    T: Copy + fmt::Display + marker::Send + 'static,
{
    fn multi_all_fill_screen(&self, color: T) {
        let screen = &self.screen;
        let handler: Vec<thread::JoinHandle<()>> = (0..screen.len() as usize)
            .map(|row| {
                let row = &screen[row];
                let row = Arc::clone(row);
                thread::spawn(move || {
                    let mut row = row.lock().unwrap();
                    row.iter_mut().for_each(|pix| pixel_changer(pix, color));
                })
            })
            .collect();
        for handle in handler {
            let _ = handle.join().unwrap();
        }
    }
    fn multi_box_screen(
        &self,
        first_x: usize,
        first_y: usize,
        last_x: usize,
        last_y: usize,
        coler: T,
    ) -> Result<(), String> {
        let screen = &self.screen;
        let max_x = screen.len();
        let max_y = screen[0].lock().unwrap().len();
        if first_y > last_y && last_y > max_y {
            return Err(String::from("The specified size is too large."));
        }
        if first_x > last_x && last_x > max_x {
            return Err(String::from("The specified size is too large."));
        }
        let handler: Vec<thread::JoinHandle<()>> = (first_y..last_y as usize)
            .map(|row| {
                let row = Arc::clone(&screen[row]);
                return thread::spawn(move || {
                    let mut row = row.lock().unwrap();
                    (first_x..last_x as usize).for_each(|pix| {
                        let pixel = &mut row[pix];
                        *pixel = coler;
                    })
                });
            })
            .collect();
        for handle in handler {
            let _ = handle.join().unwrap();
        }
        return Ok(());
    }
}
impl ScreenBuffer<u8> {
    fn new_8bit(x: usize, y: usize, c: u8) -> ScreenBuffer<u8> {
        ScreenBuffer::new(x, y, c)
    }
}
impl<T> ScreenBuffer<T>
where
    T: Copy + fmt::Display,
{
    fn new(x: usize, y: usize, color: T) -> ScreenBuffer<T> {
        ScreenBuffer {
            screen: (0..y as usize)
                .map(|_| Arc::new(Mutex::new(vec![color; x])))
                .collect(),
        }
    }
    fn display(&self) {
        for row in &self.screen {
            let row = Arc::clone(row);
            let row = row.lock().unwrap();
            row.iter().for_each(|pix| {
                print!("{}", pix);
            });
            println!("");
        }
    }
    fn single_all_fill_screen(&self, color: T) {
        for row in &self.screen {
            let row = Arc::clone(row);
            let mut row = row.lock().unwrap();
            row.iter_mut().for_each(|pix| pixel_changer(pix, color));
        }
    }
    fn single_box_screen(
        &self,
        first_x: usize,
        first_y: usize,
        last_x: usize,
        last_y: usize,
        coler: T,
    ) -> Result<(), String> {
        let screen = &self.screen;
        let max_x = screen.len();
        let max_y = screen[0].lock().unwrap().len();
        if first_y > last_y && last_y > max_y {
            return Err(String::from("The specified size is too large."));
        }
        if first_x > last_x && last_x > max_x {
            return Err(String::from("The specified size is too large."));
        }
        (first_y..last_y as usize).for_each(|row| {
            let row = Arc::clone(&screen[row]);
            let mut row = row.lock().unwrap();
            (first_x..last_x as usize).for_each(|pix| {
                let pixel = &mut row[pix];
                *pixel = coler;
            })
        });
        return Ok(());
    }
}

fn pixel_changer<T: Copy>(pixel: &mut T, color: T) {
    *pixel = color;
    thread::sleep(Duration::from_millis(10));
}

struct Example {
    first_name: String,
    last_name: String,
}

impl fmt::Display for Example {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.last_name, self.first_name)
    }
}

fn main() {
    let sc = ScreenBuffer::new(10, 10, SimpleColor::Blue);
    let _ = sc.multi_box_screen(2, 2, 8, 8, SimpleColor::Red);
    let _ = sc.multi_box_screen(4, 4, 6, 6, SimpleColor::Yellow);
    sc.display();
    sc.multi_all_fill_screen(SimpleColor::Brown);
    sc.display();
}
