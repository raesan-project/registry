use serde;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Exam {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Subject {
    pub id: String,
    pub exam_id: String,
    pub name: String,
    pub exam_name: String,
    pub display_name: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Chapter {
    pub id: String,
    pub subject_id: String,
    pub name: String,
    pub exam_name: String,
    pub subject_name: String,
    pub display_name: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Question {
    QuestionAnswer(QuestionAnswer),
    SingleMCQ(SingleMCQ),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QuestionAnswer {
    pub id: String,
    pub chapter_id: String,
    pub body: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SingleMCQ {
    pub id: String,
    pub chapter_id: String,
    pub body: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SingleMCQOption {
    pub id: String,
    pub single_mcq_id: String,
    pub key: String,
    pub value: String,
}
