// imports
use serde;

// ----- `Class` model struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Class {
    pub id: String,
    pub name: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Subject {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub class_id: String,
    pub class_name: i32,
    pub created_at: i64,
    pub updated_at: i64,
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
    pub created_at: i64,
    pub updated_at: i64,
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
    pub created_at: i64,
    pub updated_at: i64,
}
