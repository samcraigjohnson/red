extern crate termion;

use std::io::{self, Write, Read};
use termion::event::Key;
use termion::{color, style, clear, cursor};
use termion::raw::IntoRawMode;
use termion::input::TermRead;

fn main() {
    // Setup
    let mut stdout = io::stdout().into_raw_mode()
        .expect("Cannot get raw stdout");
    let stdin = io::stdin();

    // Start Editing
    init(&mut stdout);
    run(&mut stdout, stdin);
}

// Setup the way the editor looks
fn init<W: Write>(out: &mut W) {
    write!(out, "{}{}{}Realtime EDitor{}\n{}",
           clear::All,
           cursor::Goto(1,1),
           color::Fg(color::Red),
           style::Reset,
           cursor::Goto(1,2))
        .expect("Cannot open TTY");
    out.flush().expect("Cannot flush message");
}

// Read characters as the user types them.
// Quit on ESC
fn run<W: Write, R: Read>(out: &mut W, input: R) {
    let mut cx = 1;
    let mut cy = 2;

    for c in input.keys() {
        write!(out, "{}", termion::cursor::Goto(cx, cy)).
            expect("Could not move cursor");

        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char(c) => {
                write!(out, "{}", c).unwrap();
                cx += 1;
            },
            Key::Alt(c) => println!("^{}", c),
            Key::Ctrl(c) => println!("*{}", c),
            Key::Left => println!("←"),
            Key::Right => println!("→"),
            Key::Up => {
                if cy > 1 {
                    cy -= 1;
                }
            },
            Key::Down => {
                cy += 1;
            },
            Key::Backspace => println!("×"),
            _ => {},
        }
        
        out.flush().unwrap()
    }
}

// Read a single line of input from a given prompt
#[allow(dead_code)]
fn get_line<W: Write>(prompt: &[u8], buf: &mut String, out: &mut W) -> String {
    // Prompt user
    out.write(prompt).expect("Could not write to Stdout");
    out.flush().expect("Count not flush");

    // Read output
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Could not read line");
    buf.push_str(&input);
    input
}
