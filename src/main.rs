mod app;
mod enums;
mod lexer;
mod parser;

use app::App;

fn main() {
    let app = App::new();
    app.run();
}
