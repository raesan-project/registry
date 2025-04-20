use crate::{registry::reg_models, tables};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateTestMetadata {
    pub _data: Vec<reg_models::Exam>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TestInput {
    pub exams: Vec<tables::Exam>,
    pub subjects: Vec<tables::Subject>,
    pub chapters: Vec<tables::Chapter>,
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
