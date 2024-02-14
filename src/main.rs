mod app;
mod enums;
mod lexer;
mod parser;
mod vm;

use app::App;

fn main() {
    let app = App::new();
    app.run();
}
