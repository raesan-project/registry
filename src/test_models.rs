use crate::registry::reg_models;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateTestMetadata {
    pub _data: Vec<reg_models::Exam>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TestInput {
    pub exam: String,
    pub subjects: Vec<String>,
    pub chapters: Vec<String>,
    pub format: TestFormatInput,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TestFormatInput {
    pub total_questions: Vec<u32>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Test {
    pub id: String,
    pub exam_name: String,
    pub time: i64,
    pub questions: Vec<reg_models::Question>,
}
