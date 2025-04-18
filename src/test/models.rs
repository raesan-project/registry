use crate::models;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateTestPageMeta {
    pub exams: Vec<models::Exam>,
    pub subjects: Vec<models::Subject>,
    pub chapters: Vec<models::Chapter>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateTestInput {
    pub curr_step: u32,
    pub exams: Vec<models::Exam>,
    pub subjects: Vec<models::Subject>,
    pub chapters: Vec<models::Chapter>,
    pub format: TestFormatInput,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TestFormatInput {
    pub total_questions: Vec<u32>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Test {
    pub id: String,
    pub name: String,
    pub date: i64,
    pub questions: Vec<TestQuestion>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TestQuestion {
    pub id: String,
    pub body: String,
}
