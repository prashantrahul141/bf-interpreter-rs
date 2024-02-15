mod app;
mod enums;
mod lexer;
mod parser;
mod vm;

use app::App;

fn main() {
    // Create top level app.
    let app = App::new();

    // run app.
    app.run();
}
