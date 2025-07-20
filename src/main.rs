mod app;
mod formatter;
mod lesson_types;
mod lessons;
mod ui;
mod validation;

use app::App;
use macros::parse_answers;

#[parse_answers]
fn main() {
    let mut app = App::new(lessons);
    app.run();
}
