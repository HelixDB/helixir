use std::{
    env::temp_dir,
    fs::{self, read_to_string},
    io::{self},
    process::Command,
    time::Instant,
    usize,
};

mod cli;
mod lesson_types;
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

#[tokio::main]
async fn main() {
    let mut current_lesson = 0;
    let max_lessons = std::fs::read_dir("lesson_answers")
        .map(|entries| entries.count())
        .unwrap_or(0);

    if check_helix_init() {
        current_lesson = 1;
        display_lesson(current_lesson);
    } else {
        welcome_screen();
    }
    loop {
        let command = get_user_input();
        let action = parse_command(&command, current_lesson);
        match action {
            Ok(action) => match handle_action(action, current_lesson, max_lessons).await {
                ActionResult::Continue => {
                    display_lesson(current_lesson);
                }
                ActionResult::ChangeTo(new_lesson) => {
                    current_lesson = new_lesson;
                    display_lesson(current_lesson);
                }
                ActionResult::Exit => {
                    println!("Thanks for using Helixir :)");
                    break;
                }
            },
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

async fn handle_action(
    action: MenuAction,
    current_lesson: usize,
    max_lessons: usize,
) -> ActionResult {
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
            clear_screen();
            let lesson = get_lesson(current_lesson);

            if let Some(query_answer_path) = &lesson.query_answer {
                match get_or_prompt_instance_id() {
                    Ok(instance_id) => {
                        println!("Attempting to redeploy instance, may take a lil bit of time");
                        if !redeploy_instance(&instance_id) {
                            println!("Cannot proceed without successful redeploy");
                            return ActionResult::Continue;
                        }
                        match Command::new("helix").arg("check").output() {
                            Ok(output) if output.status.success() => {
                                println!("Helix check passed");

                                let lesson_data = fs::read_to_string(query_answer_path).unwrap();
                                let lesson_json: serde_json::Value =
                                    serde_json::from_str(&lesson_data).unwrap();

                                let queries = lesson_json["queries"].as_array().unwrap();
                                let query_test = &queries[0];

                                let query_name = query_test["query_name"].as_str().unwrap();
                                let input = query_test["input"].clone();
                                let expected = query_test["expected_output"].clone();

                                let query_instance: QueryValidator = self::QueryValidator::new();
                                let comparison = query_instance
                                    .execute_and_compare(query_name, input, expected)
                                    .await;
                                match comparison {
                                    Ok((_success, message)) => {
                                        println!("{}", message);
                                    }
                                    Err(e) => {
                                        let error_msg = e.to_string();
                                        if error_msg.contains("localhost:6969")
                                            || error_msg.contains("connection")
                                        {
                                            println!(
                                                "Connection Error: Cannot connect to HelixDB server."
                                            );
                                            println!(
                                                "Did you run 'helix deploy' to start the database server?"
                                            );
                                        } else {
                                            println!("Error: {}", e);
                                        }
                                    }
                                }
                                return ActionResult::Continue;
                            }
                            _ => {
                                println!("Helix check failed. Fix your queries.hx file");
                                return ActionResult::Continue;
                            }
                        }
                    }
                    Err(e) => {
                        println!("Error getting instance ID: {}", e);
                        return ActionResult::Continue;
                    }
                }
            }
            if let Some(expected_path) = &lesson.schema_answer {
                match (
                    ParsedSchema::from_file("helixdb-cfg/schema.hx"),
                    ParsedSchema::from_file(expected_path),
                ) {
                    (Ok(user_schema), Ok(expected_schema)) => {
                        let result = user_schema.validate_answer(&expected_schema);

                        if result.is_correct {
                            println!("Schema passed, good job!");
                        } else {
                            println!("Try again! Here is what might be wrong:");

                            if !result.missing_nodes.is_empty() {
                                println!("Missing nodes: {:?}", result.missing_nodes);
                            }
                            if !result.extra_nodes.is_empty() {
                                println!("Extra nodes: {:?}", result.extra_nodes);
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

                            if !result.missing_edges.is_empty() {
                                println!("Missing edges: {:?}", result.missing_edges);
                            }
                            if !result.extra_edges.is_empty() {
                                println!("Extra edges: {:?}", result.extra_edges);
                            }
                            if !result.edge_errors.is_empty() {
                                println!("Edge errors:");
                                for (edge, errors) in &result.edge_errors {
                                    println!("Edge '{}': ", edge);
                                    if let Some((user_from, expected_from)) =
                                        &errors.from_type_mismatch
                                    {
                                        println!(
                                            "From type mismatch: expected '{}', got '{}'",
                                            expected_from, user_from
                                        );
                                    }
                                    if let Some((user_to, expected_to)) = &errors.to_type_mismatch {
                                        println!(
                                            "To type mismatch: expected '{}', got '{}'",
                                            expected_to, user_to
                                        );
                                    }
                                    if !errors.property_errors.missing.is_empty() {
                                        println!(
                                            "Missing properties: {:?}",
                                            errors.property_errors.missing
                                        );
                                    }
                                    if !errors.property_errors.extra.is_empty() {
                                        println!(
                                            "Extra properties: {:?}",
                                            errors.property_errors.extra
                                        );
                                    }
                                }
                            }

                            if !result.missing_vectors.is_empty() {
                                println!("Missing vectors: {:?}", result.missing_vectors);
                            }
                            if !result.extra_vectors.is_empty() {
                                println!("Extra vectors: {:?}", result.extra_vectors);
                            }
                            if !result.vector_errors.is_empty() {
                                println!("Vector errors:");
                                for (vector, errors) in &result.vector_errors {
                                    println!("Vector '{}': ", vector);
                                    if !errors.missing.is_empty() {
                                        println!("Missing properties: {:?}", errors.missing);
                                    }
                                    if !errors.extra.is_empty() {
                                        println!("Extra properties: {:?}", errors.extra);
                                    }
                                }
                            }
                        }
                    }
                    (Err(e), _) => println!("Could not load your schema: {}", e),
                    (_, Err(e)) => println!("Could not load expected schema: {}", e),
                }
                return ActionResult::Continue;
            } else if current_lesson == 0 {
                match Command::new("helix").arg("check").output() {
                    Ok(output) if output.status.success() => {
                        println!("Helix initialization completed");
                        return ActionResult::ChangeTo(current_lesson + 1);
                    }
                    _ => {
                        println!("Helix initialization: Run 'helix init' to continue");
                        return ActionResult::Continue;
                    }
                }
            } else {
                return ActionResult::Continue;
            }
        }
        MenuAction::Help => {
            clear_screen();
            let lesson_hints = get_lesson(current_lesson).hints;
            println!("HINT:");
            lesson_hints.iter().for_each(|hint| println!("{}", hint));

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
