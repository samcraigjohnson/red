use std::io::{self, Write, Read};
use termion::event::Key;
use termion::{color, style, clear, cursor};
use termion::raw::{IntoRawMode};
use termion::input::TermRead;

pub struct Editor {
    cx: u16,
    cy: u16,
    output: termion::raw::RawTerminal,
    input: io::Stdin,
}

impl Editor {
    pub fn new() -> Editor {
        let mut stdout = io::stdout().into_raw_mode()
            .expect("Cannot get raw stdout");
        let stdin = io::stdin();

        Editor {
            cx: 1,
            cy: 1,
            output: stdout,
            input: stdin,
        }
    }

    // Setup the way the editor looks
    pub fn init(&self) {
        write!(self.output, "{}{}{}Realtime EDitor{}\n{}",
               clear::All,
               cursor::Goto(self.cx, self.cy),
               color::Fg(color::Red),
               style::Reset,
               cursor::Goto(self.cx, self.cy + 1))
            .expect("Cannot open TTY");
        self.cy += 1;
        self.flush();
    }

    // Read characters as the user types them.
    // Quit on ESC
    pub fn run(&self) {
        for c in self.input.keys() {
            write!(self.output, "{}",
                   cursor::Goto(self.cx, self.cy)).
                expect("Could not move cursor");

            match c.unwrap() {
                Key::Char('q') => break,
                Key::Char(c) => {
                    write!(self.output, "{}", c).unwrap();
                    self.cx += 1;
                },
                Key::Alt(c) => println!("^{}", c),
                Key::Ctrl(c) => println!("*{}", c),
                Key::Left => println!("←"),
                Key::Right => println!("→"),
                Key::Up => {
                    if self.cy > 1 {
                        self.cy -= 1;
                    }
                },
                Key::Down => {
                    self.cy += 1;
                },
                Key::Backspace => println!("×"),
                _ => {},
            }
            
            self.flush();
        }
    }

    // Flush output to the screen
    fn flush(&self) {
        self.output.flush().expect("Cannot flush message");
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

