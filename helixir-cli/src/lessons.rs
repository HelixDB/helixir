use std::path::Path;

pub struct Lesson {
    pub id: usize,
    pub title: String,
    pub instructions: String,
    pub hints: Vec<String>,
}

pub fn get_lesson(lesson_id: usize) -> Lesson {
    match lesson_id {
        0 => Lesson {
            id: 0,
            title: "Setup - Initialize HelixDB".to_string(),
            instructions: "Run 'helix init' to set up your helix instance".to_string(),
            hints: vec!["Check if helixdb-cfg folder exists".to_string()],
        },
        1 => Lesson {
            id: 1,
            title: "Schema Design - Nodes".to_string(),
            instructions: "Write the nodes for continents, countries, and cities. Write your schema in the newly created schema.hx file in the helixdb-cfg directory.".to_string(),
            hints: vec!["Use N:: for nodes".to_string()],
        },
        _ => Lesson {
            id: lesson_id,
            title: "Lesson Not Found".to_string(),
            instructions: "This lesson hasn't been implemented yet.".to_string(),
            hints: vec!["Try going back to a previous lesson.".to_string()],
        }
    }
}
