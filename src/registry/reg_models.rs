use serde;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Exam {
    #[serde(default)]
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub subjects: Vec<Subject>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Subject {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub exam_id: String,
    pub name: String,
    #[serde(default)]
    pub chapters: Vec<Chapter>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Chapter {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub subject_id: String,
    pub name: String,
    #[serde(default)]
    pub questions: Vec<Question>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Question {
    QuestionAnswer(QuestionAnswer),
    SingleMCQ(SingleMCQ),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QuestionAnswer {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub chapter_id: String,
    pub body: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SingleMCQ {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub chapter_id: String,
    pub body: String,
    pub options: Vec<SingleMCQOption>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SingleMCQOption {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub single_mcq_id: String,
    pub key: String,
    pub value: String,
}
