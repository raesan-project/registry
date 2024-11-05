// imports
use serde;

// ----- `Class` model struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Class {
    pub id: String,
    pub name: i32,
}

// ----- `Subject` model struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Subject {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub class_id: String,
    pub class_name: i32,
}

// ----- `Chapter` model struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Chapter {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub subject_id: String,
    pub subject_name: String,
    pub class_name: i32,
}

// ----- `Question` model struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Question {
    pub id: String,
    pub body: String,
    pub chapter_name: String,
    pub subject_name: String,
    pub class_name: i32,
    pub chapter_id: String,
}
