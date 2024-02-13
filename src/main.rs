mod app;
mod lexer;
mod parser;
mod token;

use app::App;

fn main() {
    let app = App::new();
    app.run();
}
