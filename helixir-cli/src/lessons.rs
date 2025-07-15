#[derive(Debug)]
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
            instructions: "Run 'helix init' to set up your helix instance (you can run it straight in this CLI)".into(),
            hints: vec!["Check if helixdb-cfg folder exists".into()],
            schema_answer: None,
            query_answer: None,
        },
        1 => Lesson {
            id: 1,
            title: "Schema Design - Nodes".into(),
            instructions: "We will be using HelixDB to model the relationships between continents, countries, and cities as a graph.\nFirst, we have to define what kind of entities/nodes will be in our graph.\nWe will start with 3 types of nodes: continents, countries, and cities.\nThe continent node will have a `name` property, which takes a `String`.\nThe country node will have a `name` property, a `currency` property, a `population` property (`U64`), and a `gdp` property (`F64`).\nThe city node will have a `name` property, a `description` property, and a `zip_codes` property that takes an array of strings.\n\nCreate a Continent , Country , and City node with their respective properties in schema.hx".into(),
            hints: vec!["Use N:: for nodes".into()],
            schema_answer: Some("lesson_answers/lesson1_schema.hx".into()),
            query_answer: None,
        },
        2 => Lesson {
            id: 2,
            title: "Adding in Edges".into(),
            instructions: "Now that we know what type of nodes are in our schema, we will define the relationships between those nodes.\n For this example, there is a hierarchical pattern where a city is in a country and a country is in a continent.\n\nCreate a Continent_to_Country and Country_to_City edge connecting their respective nodes with no properties in schema.hx.".into(),
            hints: vec!["Use E:: for edges".into()],
            schema_answer: Some("lesson_answers/lesson2_schema.hx".into()),
            query_answer: None,
        },
        3 => Lesson {
            id: 3,
            title: "Meta Relationships".into(),
            instructions: "In addition to the structural relationships between the nodes, you can also define relationships based on metadata. For example, a country must have a capital city.\n\nCreate a Country_to_Capital edge connecting Country to City in schema.hx".into(),
            hints: vec!["Use E:: for edges".into()],
            schema_answer: Some("lesson_answers/lesson3_schema.hx".into()),
            query_answer: None,
        },
        4 => Lesson {
            id: 4,
            title: "Defining Vectors".into(),
            instructions: "Vectors in HelixDB allow us to create vector-based searches for semantic similarity.\nA vector is an array of floating-point numbers that represents the semantic meaning of data.\nIn this case, we'll create a vector for city descriptions.\n\nCreate a CityDescription vector with vector property that takes an array of F64".into(),
            hints: vec!["Use E:: for edges".into()],
            schema_answer: Some("lesson_answers/lesson4_schema.hx".into()),
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
