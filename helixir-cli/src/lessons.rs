pub struct Lesson {
    pub id: usize,
    pub title: String,
    pub instructions: String,
    pub hints: Vec<String>,
    pub schema_answer: Option<String>,
    pub query_answer: Option<String>,
}

pub fn get_lesson(lesson_id: usize) -> Lesson {
    match lesson_id {
        0 => Lesson {
            id: 0,
            title: "Setup - Initialize HelixDB".into(),
            instructions: "Run 'helix init' to set up your helix instance".into(),
            hints: vec!["Check if helixdb-cfg folder exists".into()],
            schema_answer: None,
            query_answer: None,
        },
        1 => Lesson {
            id: 1,
            title: "Schema Design - Nodes".into(),
            instructions: "Write the nodes for continents, countries, and cities. Write your schema in the newly created schema.hx file in the helixdb-cfg directory.".into(),
            hints: vec!["Use N:: for nodes".into()],
            schema_answer: Some("lesson_answers/lesson1_schema.hx".into()),
            query_answer: None,
        },
        _ => Lesson {
            id: lesson_id,
            title: "Lesson Not Found".into(),
            instructions: "This lesson hasn't been implemented yet.".into(),
            hints: vec!["Try going back to a previous lesson.".into()],
            schema_answer: None,
            query_answer: None,
        }
    }
}
