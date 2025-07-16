use std::io;
use crate::lessons::get_lesson;

pub fn clear_screen() {
    clearscreen::clear().expect("Failed to clear screen");
}

pub fn welcome_screen() {
    let current_lesson = 0;
    clear_screen();
    println!(
        r"██╗    ██╗███████╗██╗      ██████╗ ██████╗ ███╗   ███╗███████╗    ████████╗ ██████╗ "
    );
    println!(
        r"██║    ██║██╔════╝██║     ██╔════╝██╔═══██╗████╗ ████║██╔════╝    ╚══██╔══╝██╔═══██╗"
    );
    println!(
        r"██║ █╗ ██║█████╗  ██║     ██║     ██║   ██║██╔████╔██║█████╗         ██║   ██║   ██║"
    );
    println!(
        r"██║███╗██║██╔══╝  ██║     ██║     ██║   ██║██║╚██╔╝██║██╔══╝         ██║   ██║   ██║"
    );
    println!(
        r"╚███╔███╔╝███████╗███████╗╚██████╗╚██████╔╝██║ ╚═╝ ██║███████╗       ██║   ╚██████╔╝"
    );
    println!(
        r" ╚══╝╚══╝ ╚══════╝╚══════╝ ╚═════╝ ╚═════╝ ╚═╝     ╚═╝╚══════╝       ╚═╝    ╚═════╝ "
    );
    println!("");
    println!(r"██╗  ██╗███████╗██╗     ██╗██╗  ██╗██╗██████╗ ");
    println!(r"██║  ██║██╔════╝██║     ██║╚██╗██╔╝██║██╔══██╗");
    println!(r"███████║█████╗  ██║     ██║ ╚███╔╝ ██║██████╔╝");
    println!(r"██╔══██║██╔══╝  ██║     ██║ ██╔██╗ ██║██╔══██╗");
    println!(r"██║  ██║███████╗███████╗██║██╔╝ ██╗██║██║  ██║");
    println!(r"╚═╝  ╚═╝╚══════╝╚══════╝╚═╝╚═╝  ╚═╝╚═╝╚═╝  ╚═╝");
    println!("═══════════════════════════════════════");
    println!("A rustling-styled interactive learning tool for mastering helix-db from 0 to hero!");
    println!();
    println!("Let's begin your journey!");
    display_lesson(current_lesson);
}

pub fn display_lesson(lesson_id: usize) {
    let lesson = get_lesson(lesson_id);

    println!("═══════════════════════════════════════");
    println!("Lesson {}: {}", lesson.id, lesson.title);
    println!("═══════════════════════════════════════");
    println!("{}", lesson.instructions);
    println!();
    println!();
    println!("Commands: (n)ext, (b)ack, (c)heck, (h)elp, (q)uit");
}

pub fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    input
}
