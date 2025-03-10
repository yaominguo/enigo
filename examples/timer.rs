use enigo::{
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Settings,
};
use std::{
    thread,
    time::{Duration, Instant},
};

fn main() {
    env_logger::init();
    thread::sleep(Duration::from_secs(2));
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    let now = Instant::now();

    // write text
    enigo.text("Hello World! ❤️").unwrap();

    let time = now.elapsed();
    println!("{time:?}");

    // select all
    enigo.key(Key::Control, Press).unwrap();
    enigo.key(Key::Unicode('a'), Click).unwrap();
    enigo.key(Key::Control, Release).unwrap();
}
