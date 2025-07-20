use crate::formatter::HelixFormatter;
use crate::lessons::get_lesson;
use std::io;

pub fn clear_screen() {
    clearscreen::clear().expect("Failed to clear screen");
}

#[allow(dead_code)]
pub fn welcome_screen() {
    let current_lesson = 0;
    clear_screen();

    let formatter = HelixFormatter::new();
    formatter.display_welcome();
    display_lesson(current_lesson);
}

pub fn display_lesson(lesson_id: usize) {
    let lesson = get_lesson(lesson_id);
    let formatter = HelixFormatter::new();

    formatter.display_lesson(&lesson.title, lesson.id, &lesson.instructions);
}

pub fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    input
}
