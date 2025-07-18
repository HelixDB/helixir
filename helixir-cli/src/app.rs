use crate::formatter::HelixFormatter;
use crate::lessons::get_lesson;
use crate::ui::{clear_screen, display_lesson, get_user_input};
use crate::validation::{
    ParsedQueries, ParsedSchema, QueryValidator, check_helix_init, get_completed_lessons,
    get_current_lesson, get_or_prompt_instance_id, mark_lesson_completed, redeploy_instance,
    save_current_lesson,
};
use colored::*;
use std::{fs, process::Command};

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
    GoToLesson(usize),
    RunPreviousLessons,
    ShowProgress,
}

pub struct App {
    current_lesson: usize,
    max_lessons: usize,
    formatter: HelixFormatter,
    output_messages: Vec<String>,
}

impl App {
    pub fn new() -> Self {
        let max_lessons = std::fs::read_dir("lesson_answers")
            .map(|entries| entries.count())
            .unwrap_or(0);

        Self {
            current_lesson: 0,
            max_lessons,
            formatter: HelixFormatter::new(),
            output_messages: Vec::new(),
        }
    }

    pub fn initialize(&mut self) {
        self.formatter.display_welcome();
        if check_helix_init() {
            self.current_lesson = get_current_lesson();
            self.show_welcome_menu(true);
        } else {
            self.show_welcome_menu(false);
        }
    }

    pub fn run(&mut self) {
        self.initialize();
        let initial_selection = self.get_welcome_input();
        self.handle_welcome_selection(initial_selection);

        let runtime = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
        loop {
            let command = get_user_input();
            let action = self.parse_command(&command);
            match action {
                Ok(action) => {
                    self.clear_output();
                    let result = runtime.block_on(self.handle_action(action));
                    match result {
                        ActionResult::Continue => {
                            self.display_current_lesson();
                        }
                        ActionResult::ChangeTo(new_lesson) => {
                            self.current_lesson = new_lesson;
                            let _ = save_current_lesson(self.current_lesson);
                            self.clear_output();
                            self.display_current_lesson();
                        }
                        ActionResult::Exit => {
                            self.formatter.display_info("Thanks for using Helixir :)");
                            break;
                        }
                    }
                }
                Err(error) => {
                    clear_screen();
                    self.clear_output();
                    self.add_output(format!("[ERROR] {}", error));
                    self.display_current_lesson();
                }
            }
        }
    }

    fn parse_command(&self, input: &str) -> Result<MenuAction, String> {
        let trimmed = input.trim();

        if self.current_lesson == 0 && trimmed == "helix init" {
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
            "p" => Ok(MenuAction::ShowProgress),
            "r" => Ok(MenuAction::RunPreviousLessons),
            cmd if cmd.starts_with("g ") => {
                let lesson_str = cmd.strip_prefix("g ").unwrap();
                match lesson_str.parse::<usize>() {
                    Ok(lesson_id) if lesson_id <= self.max_lessons => {
                        Ok(MenuAction::GoToLesson(lesson_id))
                    }
                    Ok(_) => Err(format!(
                        "Lesson {} is out of range (0-{})",
                        lesson_str, self.max_lessons
                    )),
                    Err(_) => Err(format!("Invalid lesson number: {}", lesson_str)),
                }
            }
            _ => Err(format!("Invalid command: {}", input)),
        }
    }

    async fn handle_action(&mut self, action: MenuAction) -> ActionResult {
        match action {
            MenuAction::Back => {
                if self.current_lesson == 0 {
                    clear_screen();
                    self.add_output(
                        "You are already at the first lesson, you cant go back any further."
                            .to_string(),
                    );
                    return ActionResult::Continue;
                }
                clear_screen();
                ActionResult::ChangeTo(self.current_lesson - 1)
            }
            MenuAction::Check => {
                clear_screen();
                let lesson = get_lesson(self.current_lesson);

                if let Some(query_answer_path) = &lesson.query_answer {
                    if let Some(expected_query_file) = &lesson.query_answer_file {
                        match (
                            ParsedQueries::from_file("helixdb-cfg/queries.hx"),
                            ParsedQueries::from_file(expected_query_file),
                        ) {
                            (Ok(user_queries), Ok(expected_queries)) => {
                                let validation_result =
                                    user_queries.validate_against(&expected_queries);

                                if !validation_result.is_correct {
                                    self.add_output("[INCORRECT] Query validation failed. Please fix your queries.hx file".to_string());

                                    if !validation_result.missing_queries.is_empty() {
                                        self.add_output(format!(
                                            "[ERROR] Missing queries: {:?}",
                                            validation_result.missing_queries
                                        ));
                                    }
                                    if !validation_result.extra_queries.is_empty() {
                                        self.add_output(format!(
                                            "[ERROR] Extra queries: {:?}",
                                            validation_result.extra_queries
                                        ));
                                    }
                                    for (query_name, error) in &validation_result.query_errors {
                                        self.add_output(format!(
                                            "[ERROR] Query '{}': {}",
                                            query_name, error
                                        ));
                                    }
                                    return ActionResult::Continue;
                                }
                                self.add_output(
                                    "[CORRECT] Query structure validation passed".to_string(),
                                );
                            }
                            (Err(e), _) => {
                                self.add_output(format!(
                                    "[ERROR] Could not parse your queries.hx file: {}",
                                    e
                                ));
                                return ActionResult::Continue;
                            }
                            (_, Err(e)) => {
                                self.add_output(format!(
                                    "[ERROR] Could not parse expected queries file: {}",
                                    e
                                ));
                                return ActionResult::Continue;
                            }
                        }
                    }

                    match get_or_prompt_instance_id() {
                        Ok(instance_id) => {
                            self.add_output("Attempting to redeploy instance".to_string());
                            if !redeploy_instance(&instance_id) {
                                self.add_output(
                                    "[ERROR] Cannot proceed without successful redeploy"
                                        .to_string(),
                                );
                                return ActionResult::Continue;
                            }
                            match Command::new("helix").arg("check").output() {
                                Ok(output) if output.status.success() => {
                                    self.add_output("[CORRECT] Helix check passed".to_string());

                                    let lesson_data =
                                        fs::read_to_string(query_answer_path).unwrap();
                                    let lesson_json: serde_json::Value =
                                        serde_json::from_str(&lesson_data).unwrap();

                                    let queries = lesson_json["queries"].as_array().unwrap();
                                    for (index, query_test) in queries.iter().enumerate() {
                                        let query_name = query_test["query_name"].as_str().unwrap();
                                        let input = query_test["input"].clone();

                                        self.add_output(format!(
                                            "Testing query {} of {}: {}",
                                            index + 1,
                                            queries.len(),
                                            query_name
                                        ));
                                        let query_instance: QueryValidator = QueryValidator::new();
                                        let comparison = query_instance
                                            .execute_and_compare(query_name, input)
                                            .await;
                                        match comparison {
                                            Ok((success, message)) => {
                                                let status = if success {
                                                    "[CORRECT]"
                                                } else {
                                                    "[INCORRECT]"
                                                };
                                                self.add_output(format!(
                                                    "{} Query {}: {}",
                                                    status, query_name, message
                                                ));
                                                if !success {
                                                    return ActionResult::Continue;
                                                }
                                            }
                                            Err(e) => {
                                                let error_msg = e.to_string();
                                                if error_msg
                                                    .contains("error decoding response body")
                                                {
                                                    self.add_output(format!(
                                                        "[ERROR] Deserialization error in query {}: {}",
                                                        query_name, error_msg
                                                    ));
                                                    self.add_output(
                                                        "[ERROR] Check if server response format matches lesson_types.rs structures".to_string()
                                                    );
                                                }
                                                return ActionResult::Continue;
                                            }
                                        }
                                    }
                                    let _ = mark_lesson_completed(self.current_lesson);
                                    self.add_output(
                                        "[CORRECT] Lesson completed! Great job!".to_string(),
                                    );
                                    return ActionResult::Continue;
                                }
                                _ => {
                                    self.add_output(
                                        "[ERROR] Helix check failed. Fix your queries.hx file"
                                            .to_string(),
                                    );
                                    return ActionResult::Continue;
                                }
                            }
                        }
                        Err(e) => {
                            self.add_output(format!("[ERROR] Error getting instance ID: {}", e));
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
                                let _ = mark_lesson_completed(self.current_lesson);
                                self.add_output(
                                    "[CORRECT] Schema passed, good job! Lesson completed!"
                                        .to_string(),
                                );
                            } else {
                                self.add_output(
                                    "[INCORRECT] Try again! Here is what might be wrong:"
                                        .to_string(),
                                );

                                if !result.missing_nodes.is_empty() {
                                    self.add_output(format!(
                                        "[ERROR] Missing nodes: {:?}",
                                        result.missing_nodes
                                    ));
                                }
                                if !result.property_errors.is_empty() {
                                    self.add_output("[ERROR] Property errors:".to_string());
                                    for (node, errors) in &result.property_errors {
                                        self.add_output(format!("[ERROR] Node '{}': ", node));
                                        if !errors.missing.is_empty() {
                                            self.add_output(format!(
                                                "[ERROR] Missing properties: {:?}",
                                                errors.missing
                                            ));
                                        }
                                        if !errors.extra.is_empty() {
                                            self.add_output(format!(
                                                "[ERROR] Extra properties: {:?}",
                                                errors.extra
                                            ));
                                        }
                                    }
                                }

                                if !result.missing_edges.is_empty() {
                                    self.add_output(format!(
                                        "[ERROR] Missing edges: {:?}",
                                        result.missing_edges
                                    ));
                                }
                                if !result.edge_errors.is_empty() {
                                    self.add_output("[ERROR] Edge errors:".to_string());
                                    for (edge, errors) in &result.edge_errors {
                                        self.add_output(format!("[ERROR] Edge '{}': ", edge));
                                        if let Some((user_from, expected_from)) =
                                            &errors.from_type_mismatch
                                        {
                                            self.add_output(format!(
                                                "[ERROR] From type mismatch: expected '{}', got '{}'",
                                                expected_from, user_from
                                            ));
                                        }
                                        if let Some((user_to, expected_to)) =
                                            &errors.to_type_mismatch
                                        {
                                            self.add_output(format!(
                                                "[ERROR] To type mismatch: expected '{}', got '{}'",
                                                expected_to, user_to
                                            ));
                                        }
                                        if !errors.property_errors.missing.is_empty() {
                                            self.add_output(format!(
                                                "[ERROR] Missing properties: {:?}",
                                                errors.property_errors.missing
                                            ));
                                        }
                                        if !errors.property_errors.extra.is_empty() {
                                            self.add_output(format!(
                                                "[ERROR] Extra properties: {:?}",
                                                errors.property_errors.extra
                                            ));
                                        }
                                    }
                                }

                                if !result.missing_vectors.is_empty() {
                                    self.add_output(format!(
                                        "[ERROR] Missing vectors: {:?}",
                                        result.missing_vectors
                                    ));
                                }
                                if !result.vector_errors.is_empty() {
                                    self.add_output("[ERROR] Vector errors:".to_string());
                                    for (vector, errors) in &result.vector_errors {
                                        self.add_output(format!("[ERROR] Vector '{}': ", vector));
                                        if !errors.missing.is_empty() {
                                            self.add_output(format!(
                                                "[ERROR] Missing properties: {:?}",
                                                errors.missing
                                            ));
                                        }
                                        if !errors.extra.is_empty() {
                                            self.add_output(format!(
                                                "[ERROR] Extra properties: {:?}",
                                                errors.extra
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                        (Err(e), _) => {
                            self.add_output(format!("[ERROR] Could not load your schema: {}", e))
                        }
                        (_, Err(e)) => self
                            .add_output(format!("[ERROR] Could not load expected schema: {}", e)),
                    }
                    return ActionResult::Continue;
                } else if self.current_lesson == 0 {
                    match Command::new("helix").arg("check").output() {
                        Ok(output) if output.status.success() => {
                            let _ = mark_lesson_completed(self.current_lesson);
                            self.add_output(
                                "[CORRECT] Helix initialization completed! Lesson completed!"
                                    .to_string(),
                            );
                            return ActionResult::ChangeTo(self.current_lesson + 1);
                        }
                        _ => {
                            self.add_output(
                                "Helix initialization: Run 'helix init' to continue".to_string(),
                            );
                            return ActionResult::Continue;
                        }
                    }
                } else {
                    return ActionResult::Continue;
                }
            }
            MenuAction::Help => {
                clear_screen();
                let lesson_hints = get_lesson(self.current_lesson).hints;
                self.formatter.print_hints(&lesson_hints);
                ActionResult::Continue
            }
            MenuAction::Next => {
                if self.current_lesson >= self.max_lessons {
                    clear_screen();
                    self.add_output(
                        "You are already at the last lesson, you cant go any further.".to_string(),
                    );
                    return ActionResult::Continue;
                }
                clear_screen();
                ActionResult::ChangeTo(self.current_lesson + 1)
            }
            MenuAction::Quit => ActionResult::Exit,
            MenuAction::GoToLesson(lesson_id) => {
                clear_screen();
                if lesson_id <= self.max_lessons {
                    self.add_output(format!("Jumping to lesson {}", lesson_id));
                    ActionResult::ChangeTo(lesson_id)
                } else {
                    self.add_output(format!(
                        "[ERROR] Lesson {} does not exist (max: {})",
                        lesson_id, self.max_lessons
                    ));
                    ActionResult::Continue
                }
            }
            MenuAction::ShowProgress => {
                clear_screen();
                self.show_progress();
                ActionResult::Continue
            }
            MenuAction::RunPreviousLessons => {
                clear_screen();
                self.run_previous_lessons().await
            }
        }
    }
    fn clear_output(&mut self) {
        self.output_messages.clear();
    }
    fn add_output(&mut self, message: String) {
        self.output_messages.push(message);
    }
    fn display_current_lesson(&self) {
        let lesson = get_lesson(self.current_lesson);
        if self.output_messages.is_empty() {
            self.formatter
                .display_lesson(&lesson.title, lesson.id, &lesson.instructions);
        } else {
            self.formatter.display_lesson_with_output(
                &lesson.title,
                lesson.id,
                &lesson.instructions,
                &self.output_messages,
            );
        }
    }

    fn show_progress(&self) {
        let completed_lessons = get_completed_lessons();
        let total_lessons = self.max_lessons + 1;

        self.formatter.display_info(&format!(
            "Progress: {} / {} lessons completed",
            completed_lessons.len(),
            total_lessons
        ));
        self.formatter
            .display_info(&format!("Current lesson: {}", self.current_lesson));

        if !completed_lessons.is_empty() {
            println!("{}", "Completed lessons:".bright_green().bold());
            for lesson_id in completed_lessons {
                let lesson = get_lesson(lesson_id);
                println!("  {} - {}", lesson_id, lesson.title.bright_white());
            }
        } else {
            println!("{}", "No lessons completed yet.".bright_yellow());
        }
        println!();
    }

    async fn run_previous_lessons(&self) -> ActionResult {
        if self.current_lesson == 0 {
            self.formatter.display_info("No previous lessons to run.");
            return ActionResult::Continue;
        }

        self.formatter.display_info(&format!(
            "Running all lessons before lesson {}...",
            self.current_lesson
        ));

        let instance_id = match get_or_prompt_instance_id() {
            Ok(id) => id,
            Err(e) => {
                self.formatter
                    .display_error(&format!("Error getting instance ID: {}", e));
                return ActionResult::Continue;
            }
        };

        for lesson_id in 0..self.current_lesson {
            let lesson = get_lesson(lesson_id);

            if lesson.query_answer.is_none() {
                continue;
            }

            self.formatter
                .display_info(&format!("Running lesson {}: {}", lesson_id, lesson.title));

            if let Some(query_answer_path) = &lesson.query_answer {
                if !redeploy_instance(&instance_id) {
                    self.formatter
                        .display_error(&format!("Failed to redeploy for lesson {}", lesson_id));
                    continue;
                }

                let lesson_data = match fs::read_to_string(query_answer_path) {
                    Ok(data) => data,
                    Err(e) => {
                        self.formatter.display_error(&format!(
                            "Could not read lesson {} queries: {}",
                            lesson_id, e
                        ));
                        continue;
                    }
                };

                let lesson_json: serde_json::Value = match serde_json::from_str(&lesson_data) {
                    Ok(json) => json,
                    Err(e) => {
                        self.formatter.display_error(&format!(
                            "Could not parse lesson {} JSON: {}",
                            lesson_id, e
                        ));
                        continue;
                    }
                };

                if let Some(queries) = lesson_json["queries"].as_array() {
                    for query_test in queries {
                        let query_name = query_test["query_name"].as_str().unwrap_or("unknown");
                        let input = query_test["input"].clone();

                        let query_instance = QueryValidator::new();
                        match query_instance.execute_and_compare(query_name, input).await {
                            Ok((success, message)) => {
                                if success {
                                    println!("  {} {}", "[OK]".bright_green().bold(), query_name);
                                } else {
                                    self.formatter.display_error(&format!(
                                        "Query {} failed: {}",
                                        query_name, message
                                    ));
                                }
                            }
                            Err(e) => {
                                self.formatter
                                    .display_error(&format!("Query {} error: {}", query_name, e));
                            }
                        }
                    }
                }
            }
            let _ = mark_lesson_completed(lesson_id);
        }

        self.formatter
            .display_validation_result(true, "All previous lessons executed successfully!");
        ActionResult::Continue
    }
    fn show_welcome_menu(&self, has_progress: bool) {
        println!();
        if has_progress {
            println!("{}", "What would you like to do?".bright_white().bold());
            println!();
            println!(
                "{} {}",
                "1)".bright_green().bold(),
                format!("Resume from lesson {}", self.current_lesson).white()
            );
            println!(
                "{} {}",
                "2)".bright_green().bold(),
                "Go to specific lesson".white()
            );
            println!(
                "{} {}",
                "3)".bright_green().bold(),
                "Start from beginning".white()
            );
        } else {
            println!("{}", "What would you like to do?".bright_white().bold());
            println!();
            println!(
                "{} {}",
                "1)".bright_green().bold(),
                "Get started (Lesson 0)".white()
            );
            println!(
                "{} {}",
                "2)".bright_green().bold(),
                "Go to specific lesson".white()
            );
        }
        println!();
        print!("{}", "Enter your choice: ".bright_yellow());
    }
    fn get_welcome_input(&self) -> String {
        use std::io::{self, Write};
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }
    fn handle_welcome_selection(&mut self, selection: String) {
        clear_screen();

        match selection.as_str() {
            "1" => {
                if check_helix_init() {
                    display_lesson(self.current_lesson);
                } else {
                    self.current_lesson = 0;
                    display_lesson(self.current_lesson);
                }
            }
            "2" => {
                print!("{}", "Enter lesson number: ".bright_yellow());
                use std::io::{self, Write};
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();

                if let Ok(lesson_num) = input.trim().parse::<usize>() {
                    if lesson_num <= self.max_lessons {
                        self.current_lesson = lesson_num;
                        let _ = save_current_lesson(self.current_lesson);
                        clear_screen();
                        display_lesson(self.current_lesson);
                    } else {
                        println!(
                            "{}",
                            format!("Invalid lesson number. Max lesson is {}", self.max_lessons)
                                .bright_red()
                        );
                        self.current_lesson = 0;
                        display_lesson(self.current_lesson);
                    }
                } else {
                    println!("{}", "Invalid input. Starting from lesson 0.".bright_red());
                    self.current_lesson = 0;
                    display_lesson(self.current_lesson);
                }
            }
            "3" if check_helix_init() => {
                self.current_lesson = 0;
                let _ = save_current_lesson(self.current_lesson);
                display_lesson(self.current_lesson);
            }
            _ => {
                println!("{}", "Invalid choice. Starting from lesson 0.".bright_red());
                self.current_lesson = 0;
                display_lesson(self.current_lesson);
            }
        }
    }
}
