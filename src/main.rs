extern crate termion;

use std::io::{self, Write};
use termion::event::Key;
use termion::{color, style, clear, cursor};
use termion::raw::IntoRawMode;

fn main() {
    // Setup
    let mut stdout = io::stdout().into_raw_mode()
        .expect("Cannot get raw stdout");

    // Start Editing
    init(&mut stdout);
    read_input(&mut stdout);
}

// Setup the way the editor looks
fn init<W: Write>(out: &mut W) {
    write!(out, "{}{}\n{}Realtime EDitor{}\n",
           clear::All,
           cursor::Goto(1,1),
           color::Fg(color::Red),
           style::Reset).expect("Cannot open TTY");
    out.flush().expect("Cannot flush message");
}

// Continously read input from stdin (line-based)
fn read_input<W: Write>(out: &mut W) {
    let mut read = String::new();
    let mut counter = 1;
    loop {
        let prompt = format!("{}: ", counter);
        let line = get_line(&prompt.into_bytes(), &mut read, out);
        let trimmed_line = line.trim();

        if trimmed_line == "q" {
            break;
        } else if trimmed_line == "top" {
            println!("{}", cursor::Goto(1,1));
        }

        counter += 1;
    }
    println!("Read data:\n{}", read);
}

// Read a single line of input from a given prompt
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
