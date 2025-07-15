use std::{
    io::{self},
    process::Command,
    usize,
};

mod cli;
mod lessons;
mod validation;

use cli::*;
use lessons::*;
use validation::*;

pub enum ActionResult {
    Continue,
    ChangeTo(usize),
    Exit,
}

pub enum MenuAction {
    Next,
    Back,
    Help,
    Check,
    Quit,
}

fn main() {
    let mut current_lesson = 0;
    let max_lessons = 2;

    if check_helix_init() {
        current_lesson = 1;
    } else {
        welcome_screen();
    }
    loop {
        display_lesson(current_lesson);

        let command = get_user_input();
        let action = parse_command(&command, current_lesson);
        match action {
            Ok(action) => {
                match handle_action(action, current_lesson, max_lessons) {
                    ActionResult::Continue => {
                        // do nothing for now
                    }
                    ActionResult::ChangeTo(new_lesson) => {
                        current_lesson = new_lesson;
                    }
                    ActionResult::Exit => {
                        println!("Thanks for using Helixir :)");
                        break;
                    }
                }
            }
            Err(error) => println!("Error: {}", error),
        }
    }
}

fn display_lesson(lesson_id: usize) {
    let lesson = get_lesson(lesson_id);
    println!("═══════════════════════════════════════");
    println!("Lesson {}: {}", lesson.id, lesson.title);
    println!("═══════════════════════════════════════");
    println!("{}", lesson.instructions);
    println!();
    println!("Commands: (n)ext, (b)ack, (c)heck, (h)elp, (q)uit");
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    input
}

fn parse_command(input: &str, current_lesson: usize) -> Result<MenuAction, String> {
    let trimmed = input.trim();

    if current_lesson == 0 && trimmed == "helix init" {
        let output = Command::new("helix").arg("init").output();
        match output {
            Ok(result) => {
                if result.status.success() {
                    return Ok(MenuAction::Check);
                }
            }
            Err(_) => {
                println!("Error: Could not run 'helix check'. Make sure HelixDB is installed.");
                return Err("helix init failed".to_string());
            }
        }
    }
    match trimmed.to_lowercase().as_str() {
        "c" => Ok(MenuAction::Check),
        "h" => Ok(MenuAction::Help),
        "n" => Ok(MenuAction::Next),
        "b" => Ok(MenuAction::Back),
        "q" => Ok(MenuAction::Quit),
        _ => {
            clear_screen();
            Err(format!("Invalid command: {}", input))
        }
    }
}

fn handle_action(action: MenuAction, current_lesson: usize, max_lessons: usize) -> ActionResult {
    match action {
        MenuAction::Back => {
            if current_lesson == 0 {
                clear_screen();
                println!("You are already at the first lesson, you cant go back any further.");
                return ActionResult::Continue;
            }
            clear_screen();
            ActionResult::ChangeTo(current_lesson - 1)
        }
        MenuAction::Check => {
            let lesson = get_lesson(current_lesson);
            if let Some(expected_path) = &lesson.schema_answer {
                let user_schema = match ParsedSchema::from_file("helixdb-cfg/schema.hx") {
                    Ok(schema) => schema,
                    Err(e) => {
                        println!("Error loading your schema: {}", e);
                        return ActionResult::Continue;
                    }
                };
                let expected_schema = match ParsedSchema::from_file(expected_path) {
                    Ok(schema) => schema,
                    Err(e) => {
                        println!("Error loading your answer: {}", e);
                        return ActionResult::Continue;
                    }
                };

                let result = user_schema.validate_answer(&expected_schema);
                if result.is_correct {
                    clear_screen();
                    print!("Correct! Moving on to the next lesson!\n");
                    return ActionResult::ChangeTo(current_lesson + 1);
                } else {
                    if !result.missing_nodes.is_empty() {
                        println!("Missing nodes: {:?}", result.missing_nodes);
                    }
                    if !result.extra_nodes.is_empty() {
                        println!("Extra nodes (not expected): {:?}", result.extra_nodes);
                    }
                    if !result.property_errors.is_empty() {
                        println!("Property errors:");
                        for (node, errors) in &result.property_errors {
                            println!("Node '{}': ", node);
                            if !errors.missing.is_empty() {
                                println!("Missing properties: {:?}", errors.missing);
                            }
                            if !errors.extra.is_empty() {
                                println!("Extra properties: {:?}", errors.extra);
                            }
                        }
                    }
                    clear_screen();
                    println!("\nTry again! Use (h)elp if you need hints.");
                }
            } else {
                println!("This lesson doesn't require schema validation.");
            }
            ActionResult::Continue
        }
        MenuAction::Help => {
            // TO DO
            ActionResult::Continue
        }
        MenuAction::Next => {
            if current_lesson >= max_lessons {
                clear_screen();
                println!("You are already at the last lesson, you cant go any further.");
                return ActionResult::Continue;
            }
            clear_screen();
            ActionResult::ChangeTo(current_lesson + 1)
        }
        MenuAction::Quit => ActionResult::Exit,
    }
}

fn welcome_screen() {
    println!("\n🧬 Welcome to Helixir! 🧬");
    println!("═══════════════════════════════════════");
    println!("A rustling-styled interactive learning tool for mastering helix-db from 0 to hero!");
    println!();
    println!("Let's begin your journey! 🚀");
}
