use clearscreen::clear;

pub fn clear_screen() {
    clearscreen::clear().expect("Failed to clear screen");
}
