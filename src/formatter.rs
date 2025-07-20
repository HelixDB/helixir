use colored::*;
use textwrap::{Options, wrap};

pub struct HelixFormatter {
    #[allow(dead_code)]
    terminal_width: usize,
}

impl HelixFormatter {
    pub fn new() -> Self {
        Self { terminal_width: 80 }
    }
    pub fn display_lesson(&self, title: &str, lesson_id: usize, instructions: &str) {
        self.print_lesson_header(title, lesson_id);
        self.print_lesson_content(instructions);
        self.print_commands();
    }
    pub fn display_lesson_with_output(
        &self,
        title: &str,
        lesson_id: usize,
        instructions: &str,
        output_messages: &[String],
    ) {
        self.print_lesson_header(title, lesson_id);
        self.print_lesson_content(instructions);

        if !output_messages.is_empty() {
            self.print_output_section(output_messages);
        }

        self.print_commands();
    }
    fn print_output_section(&self, messages: &[String]) {
        println!();
        println!("{}", "OUTPUT".truecolor(238, 212, 159).bold());
        println!("{}", "─".repeat(50).truecolor(238, 212, 159));

        for message in messages {
            if message.contains("[INCORRECT]") {
                let colored_message = message.replace(
                    "[INCORRECT]",
                    &"[INCORRECT]".truecolor(237, 135, 150).bold().to_string(),
                );
                println!("{}", colored_message);
            } else if message.contains("[ERROR]") {
                let colored_message = message.replace(
                    "[ERROR]",
                    &"[ERROR]".truecolor(237, 135, 150).bold().to_string(),
                );
                println!("{}", colored_message);
            } else {
                println!("{}", message);
            }
        }
        println!();
    }

    fn print_lesson_header(&self, title: &str, lesson_id: usize) {
        let header_text = format!("Lesson {}: {}", lesson_id, title);
        let border_length = std::cmp::max(header_text.len(), 50);
        let border = "═".repeat(border_length);

        println!();
        println!("{}", border.truecolor(145, 215, 227).bold());
        println!("{}", header_text.truecolor(202, 211, 245).bold());
        println!("{}", border.truecolor(145, 215, 227).bold());
        println!();
    }
    fn print_lesson_content(&self, content: &str) {
        let formatted_content = self.parse_markdown(content);
        let paragraphs: Vec<&str> = formatted_content.split("\n\n").collect();

        for paragraph in paragraphs {
            if paragraph.trim().is_empty() {
                continue;
            }

            self.format_paragraph_simple(paragraph);
            println!();
        }
    }

    fn format_paragraph_simple(&self, paragraph: &str) {
        let lines: Vec<&str> = paragraph.split('\n').collect();

        for line in lines {
            let trimmed = line.trim();

            if trimmed.is_empty() {
                continue;
            }

            if trimmed.starts_with("▶ ") {
                println!("{}", trimmed.truecolor(145, 215, 227).bold());
            } else if trimmed.starts_with("- ") {
                let parts: Vec<&str> = trimmed.split("- ").collect();
                if parts.len() > 1 {
                    println!("- {}", parts[1].truecolor(202, 211, 245));
                } else {
                    println!("{}", trimmed.truecolor(202, 211, 245));
                }
            } else {
                println!("{}", trimmed.truecolor(202, 211, 245));
            }
        }
    }

    #[allow(dead_code)]
    fn format_paragraph(&self, paragraph: &str) {
        let lines: Vec<&str> = paragraph.split('\n').collect();

        for line in lines {
            let trimmed = line.trim();

            if trimmed.is_empty() {
                continue;
            }

            if self.is_code_block(trimmed) {
                self.print_code_block(trimmed);
            } else if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
                self.print_bullet_point(trimmed);
            } else if trimmed.starts_with("Write ") || trimmed.starts_with("Create ") {
                self.print_task_instruction(trimmed);
            } else {
                self.print_regular_text(trimmed);
            }
        }
    }

    fn is_code_block(&self, line: &str) -> bool {
        line.contains("QUERY ")
            || line.contains("N::")
            || line.contains("E::")
            || line.contains("V::")
            || line.contains("AddN")
            || line.contains("AddE")
            || line.contains("AddV")
            || line.contains("helix ")
            || line.contains("schema.hx")
            || line.contains("query.hx")
            || line.contains("::WHERE")
            || line.contains("::UPDATE")
            || line.contains("::COUNT")
            || line.contains("::RANGE")
    }

    #[allow(dead_code)]
    fn print_code_block(&self, line: &str) {
        let highlighted = self.highlight_helix_syntax(line);
        println!("{}", highlighted);
    }

    fn highlight_helix_syntax(&self, code: &str) -> String {
        let mut result = code.to_string();

        let keywords = [
            "QUERY", "WHERE", "AND", "OR", "EXISTS", "COUNT", "RANGE", "UPDATE", "DROP", "AddN",
            "AddE", "AddV", "SearchV", "RETURN",
        ];

        for keyword in keywords {
            result = result.replace(
                keyword,
                &keyword.truecolor(198, 160, 246).bold().to_string(),
            );
        }

        let schema_patterns = ["N::", "E::", "V::"];
        for pattern in schema_patterns {
            result = result.replace(
                pattern,
                &pattern.truecolor(138, 173, 244).bold().to_string(),
            );
        }

        let types = ["String", "I64", "U64", "F64", "ID"];
        for type_name in types {
            result = result.replace(type_name, &type_name.truecolor(166, 218, 149).to_string());
        }

        let operators = ["::EQ", "::LT", "::GT", "::LTE", "::GTE", "::Out", "::In"];
        for op in operators {
            result = result.replace(op, &op.truecolor(238, 212, 159).to_string());
        }

        if result.contains(".hx") {
            result = result.replace(".hx", &".hx".truecolor(139, 213, 202).to_string());
        }
        if result.contains("helix ") {
            result = result.replace(
                "helix ",
                &"helix ".truecolor(237, 135, 150).bold().to_string(),
            );
        }

        result
    }

    #[allow(dead_code)]
    fn print_bullet_point(&self, line: &str) {
        let content = line.trim_start_matches("- ").trim_start_matches("* ");
        let formatted_content = self.parse_markdown(content);
        println!("{} {}", "▶".bright_blue().bold(), formatted_content);
    }

    #[allow(dead_code)]
    fn print_task_instruction(&self, line: &str) {
        let formatted_line = self.parse_markdown(line);
        if line.starts_with("Write ") {
            println!("{}", formatted_line.bright_yellow().bold());
        } else if line.starts_with("Create ") {
            println!("{}", formatted_line.bright_green().bold());
        } else {
            println!("{}", formatted_line.bright_cyan().bold());
        }
    }

    #[allow(dead_code)]
    fn print_regular_text(&self, line: &str) {
        let formatted_line = self.parse_markdown(line);
        let wrapped = wrap(&formatted_line, Options::new(self.terminal_width));
        for wrapped_line in wrapped {
            println!("{}", wrapped_line);
        }
    }

    fn parse_markdown(&self, text: &str) -> String {
        let mut result = text.to_string();
        let mut replacements = Vec::new();

        for node_type in &[
            "Country_to_Capital",
            "getCountryNames",
            "getContinentByName",
            "getCountryByName",
            "getCityByName",
            "getAllContinents",
            "getAllCities",
            "getCountriesInContinent",
            "getCitiesInContinent",
            "Out",
            "getAllCountries",
            "createContinent",
            "createCountry",
            "createCity",
            "Continent_to_Country",
            "Country_to_City",
            "CityDescription",
            "City_to_Embedding",
            "Continent",
            "Country",
            "City",
            "city",
            "country",
            "continent",
            "vector",
            "AddN",
            "AddE",
            "AddV",
            "countCapitals",
            "updateDescription",
            "deleteCity",
            "getCapital",
            "getCountriesByGdp",
            "getCountriesWithCapitals",
            "deleteCountry",
            "getContinent",
            "getCountriesByCurrency",
            "getContinentCities",
            "getCitiesInCountry",
            "updatePopGdp",
            "getCity",
            "getCountriesByCurrPop",
            "setCapital",
            "updateCurrency",
            "getCountry",
            "getCountriesByPopulation",
            "deleteCapital",
            "embedDescription",
            "updateCapital",
            "getCountriesByPopGdp",
            "getCountryByCityCnt",
            "searchDescriptions",
        ] {
            let pattern = format!(r"\b{}\b", regex::escape(node_type));
            if let Ok(regex) = regex::Regex::new(&pattern) {
                if regex.is_match(&result) {
                    let placeholder = format!("__HIGHLIGHT_{}__", replacements.len());
                    let highlighted = node_type.truecolor(238, 212, 159).bold().to_string();
                    replacements.push((placeholder.clone(), highlighted));
                    result = regex.replace_all(&result, &placeholder).to_string();
                }
            }
        }

        for (placeholder, highlighted) in replacements {
            result = result.replace(&placeholder, &highlighted);
        }
        while let Some(start) = result.find("**") {
            if let Some(end) = result[start + 2..].find("**") {
                let end = end + start + 2;
                let bold_text = &result[start + 2..end];
                let formatted = bold_text.truecolor(202, 211, 245).bold().to_string();
                result.replace_range(start..end + 2, &formatted);
            } else {
                break;
            }
        }

        result
    }

    pub fn print_hints(&self, hints: &[String]) {
        if hints.is_empty() {
            return;
        }

        println!("{}", "HINTS".truecolor(238, 212, 159).bold());
        println!("{}", "─".repeat(20).truecolor(238, 212, 159));

        for (i, hint) in hints.iter().enumerate() {
            let hint_number = format!("{}.", i + 1);
            let formatted_hint = if self.is_code_block(hint) {
                self.highlight_helix_syntax(hint)
            } else {
                self.parse_markdown(hint)
            };

            println!(
                "{} {}",
                hint_number.truecolor(238, 212, 159).bold(),
                formatted_hint
            );
        }
        println!();
    }

    fn print_commands(&self) {
        println!("{}", "COMMANDS".truecolor(166, 218, 149).bold());
        println!("{}", "─".repeat(20).truecolor(166, 218, 149));

        let commands = [
            ("n", "next", "Continue to next lesson"),
            ("b", "back", "Go to previous lesson"),
            ("c", "check", "Check your answer"),
            ("h", "help", "Show help"),
            ("g N", "goto", "Go to specific lesson (e.g., 'g 5')"),
            ("p", "progress", "Show lesson progress"),
            ("r", "run-all", "Run all previous lessons"),
            ("q", "quit", "Exit the program"),
        ];

        for (key, cmd, desc) in commands {
            println!(
                "{} {} - {}",
                format!("({})", key).truecolor(166, 218, 149).bold(),
                cmd.truecolor(202, 211, 245).bold(),
                desc.truecolor(184, 192, 224)
            );
        }
        println!();
    }

    pub fn display_welcome(&self) {
        println!();
        self.print_ascii_art();
        println!();
        println!(
            "{}",
            "  [ A rustling-styled interactive learning tool for"
                .truecolor(184, 192, 224)
                .bold()
        );
        println!(
            "{}",
            "  mastering helix-db ]".truecolor(184, 192, 224).bold()
        );
    }

    fn print_ascii_art(&self) {
        let helix_lines = [
            r"██╗  ██╗███████╗██╗     ██╗██╗  ██╗██╗██████╗ ",
            r"██║  ██║██╔════╝██║     ██║╚██╗██╔╝██║██╔══██╗",
            r"███████║█████╗  ██║     ██║ ╚███╔╝ ██║██████╔╝",
            r"██╔══██║██╔══╝  ██║     ██║ ██╔██╗ ██║██╔══██╗",
            r"██║  ██║███████╗███████╗██║██╔╝ ██╗██║██║  ██║",
            r"╚═╝  ╚═╝╚══════╝╚══════╝╚═╝╚═╝  ╚═╝╚═╝╚═╝  ╚═╝",
        ];

        for line in helix_lines {
            println!("  {}", line.truecolor(198, 160, 246).bold());
        }
    }

    pub fn display_validation_result(&self, is_correct: bool, message: &str) {
        println!();
        if is_correct {
            println!("{}", "[CORRECT]".truecolor(166, 218, 149).bold());
            println!("{}", message.truecolor(202, 211, 245));
        } else {
            println!("{}", "[INCORRECT]".truecolor(237, 135, 150).bold());
            println!("{}", message.truecolor(237, 135, 150));
        }
        println!();
    }

    pub fn display_error(&self, error: &str) {
        println!();
        println!("{}", "[ERROR]".truecolor(237, 135, 150).bold());
        println!("{}", error.truecolor(237, 135, 150));
        println!();
    }

    pub fn display_info(&self, message: &str) {
        println!();
        println!("{}", message.truecolor(138, 173, 244));
        println!();
    }
}

impl Default for HelixFormatter {
    fn default() -> Self {
        Self::new()
    }
}
