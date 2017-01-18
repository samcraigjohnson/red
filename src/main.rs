extern crate termion;

mod editor;

fn main() {
    let editor = editor::Editor::new();
    editor.init();
    editor.run();
}
