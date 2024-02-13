mod app;
mod lexer;
mod token;

use app::App;

fn main() {
    let app = App::new();
    app.run();
}
