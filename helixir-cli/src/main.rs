mod app;
mod lesson_types;
mod lessons;
mod ui;
mod validation;

use app::App;

fn main() {
    let mut app = App::new();
    app.run();
}
