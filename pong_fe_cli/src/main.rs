extern crate pancurses;

use pancurses::Input;

fn main() {
    let window = pancurses::initscr();
    pancurses::nonl();
    pancurses::noraw();
    pancurses::noecho();
    // pancurses::nocbreak();
    window.printw("Hello Rust");
    loop {
        window.refresh();
        let input = window.getch().unwrap();
        match input {
            Input::KeyAbort => break,
            Input::Character(c) => println!("{}", c),
            _ => panic!("What key was that?"),
        }
    }
    pancurses::endwin();
}
